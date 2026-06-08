import fs from 'node:fs'
import path from 'node:path'

const version = process.argv[2]

if (!version || !/^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$/.test(version)) {
  console.error('Usage: npm run version:set -- <semver>')
  process.exit(1)
}

const root = process.cwd()

function writeJson(relativePath, updater) {
  const filePath = path.join(root, relativePath)
  const data = JSON.parse(fs.readFileSync(filePath, 'utf8'))
  updater(data)
  fs.writeFileSync(filePath, `${JSON.stringify(data, null, 2)}\n`)
}

writeJson('package.json', (data) => {
  data.version = version
})

if (fs.existsSync(path.join(root, 'package-lock.json'))) {
  writeJson('package-lock.json', (data) => {
    data.version = version
    if (data.packages?.['']) {
      data.packages[''].version = version
    }
  })
}

writeJson('src-tauri/tauri.conf.json', (data) => {
  data.version = version
})

const cargoPath = path.join(root, 'src-tauri/Cargo.toml')
const cargo = fs.readFileSync(cargoPath, 'utf8')
fs.writeFileSync(
  cargoPath,
  cargo.replace(/^version = ".*"$/m, `version = "${version}"`),
)

const cargoLockPath = path.join(root, 'src-tauri/Cargo.lock')
if (fs.existsSync(cargoLockPath)) {
  const cargoLock = fs.readFileSync(cargoLockPath, 'utf8')
  fs.writeFileSync(
    cargoLockPath,
    cargoLock.replace(
      /(\[\[package\]\]\r?\nname = "maildrop"\r?\nversion = )"[^"]+"/,
      `$1"${version}"`,
    ),
  )
}

console.log(`MailDrop version set to ${version}`)
