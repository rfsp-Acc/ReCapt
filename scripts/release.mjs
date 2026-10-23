// Publica uma versao nova: atualiza o numero nos 3 arquivos, faz commit, cria a tag e envia.
// A tag dispara .github/workflows/release.yml, que gera Windows e Mac e publica em
// rfsp-Acc/ReCapt-releases; os apps instalados se atualizam sozinhos a partir de la.
//
// Uso: pnpm release 0.2.0
import { execSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";

const run = (cmd) => execSync(cmd, { stdio: "inherit" });
const out = (cmd) => execSync(cmd, { encoding: "utf8" }).trim();

const version = process.argv[2];
if (!/^\d+\.\d+\.\d+$/.test(version ?? "")) {
  console.error("Uso: pnpm release <versao>, ex.: pnpm release 0.2.0");
  process.exit(1);
}

const conf = JSON.parse(readFileSync("src-tauri/tauri.conf.json", "utf8"));
const current = conf.version;
const [next, prev] = [version, current].map((v) => v.split(".").map(Number));
if ((next[0] - prev[0] || next[1] - prev[1] || next[2] - prev[2]) <= 0) {
  console.error(`A versao ${version} precisa ser maior que a atual (${current}): o app so atualiza para versoes maiores.`);
  process.exit(1);
}
if (out("git branch --show-current") !== "main") {
  console.error("Rode na branch main.");
  process.exit(1);
}
if (out("git status --porcelain")) {
  console.error("Ha alteracoes sem commit; faca o commit antes de publicar.");
  process.exit(1);
}

conf.version = version;
writeFileSync("src-tauri/tauri.conf.json", JSON.stringify(conf, null, 2) + "\n");

const pkg = JSON.parse(readFileSync("package.json", "utf8"));
pkg.version = version;
writeFileSync("package.json", JSON.stringify(pkg, null, 2) + "\n");

const cargo = readFileSync("src-tauri/Cargo.toml", "utf8");
writeFileSync("src-tauri/Cargo.toml", cargo.replace(/^version = ".*"$/m, `version = "${version}"`));
// Atualiza o Cargo.lock com a versao nova do pacote.
run("cargo update --manifest-path src-tauri/Cargo.toml --workspace --offline");

run("git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml src-tauri/Cargo.lock");
run(`git commit -m "Versao ${version}"`);
run(`git tag v${version}`);
run("git -c credential.helper= -c credential.helper='!gh auth git-credential' push origin main --tags");
console.log(`\nVersao ${version} enviada. Acompanhe em: gh run watch -R rfsp-Acc/ReCapt`);
