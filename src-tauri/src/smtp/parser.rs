use mail_parser::{MessageParser, MimeHeaders};

pub struct ParsedMail {
    pub message_id: Option<String>,
    pub from_addr: String,
    pub from_name: Option<String>,
    pub to_addrs: Vec<String>,
    pub subject: String,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
    pub attachments: Vec<ParsedAttachment>,
}

pub struct ParsedAttachment {
    pub filename: String,
    pub content_type: String,
    pub content: Vec<u8>,
}

impl Default for ParsedMail {
    fn default() -> Self {
        Self {
            message_id: None,
            from_addr: String::new(),
            from_name: None,
            to_addrs: Vec::new(),
            subject: String::new(),
            text_body: None,
            html_body: None,
            attachments: Vec::new(),
        }
    }
}

pub fn parse(raw: &[u8]) -> ParsedMail {
    let Some(msg) = MessageParser::default().parse(raw) else {
        return ParsedMail::default();
    };

    let (from_addr, from_name) = msg
        .from()
        .and_then(|a| first_addr(a))
        .unwrap_or_else(|| (String::new(), None));

    let to_addrs = msg
        .to()
        .map(|a| all_emails(a))
        .unwrap_or_default();

    let attachments = msg
        .attachments()
        .enumerate()
        .map(|(idx, part)| {
            let content_type = part
                .content_type()
                .map(content_type_to_string)
                .unwrap_or_else(|| "application/octet-stream".to_string());
            ParsedAttachment {
                filename: part
                    .attachment_name()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("attachment-{}", idx + 1)),
                content_type,
                content: part.contents().to_vec(),
            }
        })
        .collect();

    ParsedMail {
        message_id: msg.message_id().map(|s| s.to_string()),
        from_addr,
        from_name,
        to_addrs,
        subject: msg.subject().map(|s| s.to_string()).unwrap_or_default(),
        text_body: msg.body_text(0).map(|s| s.into_owned()),
        html_body: msg.body_html(0).map(|s| s.into_owned()),
        attachments,
    }
}

fn content_type_to_string(content_type: &mail_parser::ContentType) -> String {
    match content_type.subtype() {
        Some(subtype) => format!("{}/{}", content_type.ctype(), subtype),
        None => content_type.ctype().to_string(),
    }
}

fn first_addr(addr: &mail_parser::Address) -> Option<(String, Option<String>)> {
    match addr {
        mail_parser::Address::List(list) => list.first().map(|a| {
            (
                a.address.as_deref().unwrap_or("").to_string(),
                a.name.as_deref().map(|s| s.to_string()),
            )
        }),
        mail_parser::Address::Group(groups) => groups
            .first()
            .and_then(|g| g.addresses.first())
            .map(|a| {
                (
                    a.address.as_deref().unwrap_or("").to_string(),
                    a.name.as_deref().map(|s| s.to_string()),
                )
            }),
    }
}

fn all_emails(addr: &mail_parser::Address) -> Vec<String> {
    match addr {
        mail_parser::Address::List(list) => list
            .iter()
            .filter_map(|a| a.address.as_deref().map(|s| s.to_string()))
            .collect(),
        mail_parser::Address::Group(groups) => groups
            .iter()
            .flat_map(|g| g.addresses.iter())
            .filter_map(|a| a.address.as_deref().map(|s| s.to_string()))
            .collect(),
    }
}
