import { useEffect, useRef, useState } from "react";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, Update } from "@tauri-apps/plugin-updater";
import { NoticeKind } from "./useNotices";

const CHECK_EVERY_MS = 6 * 60 * 60 * 1000;
/** Tempo para o aviso aparecer antes de o app fechar para instalar. */
const NOTICE_BEFORE_INSTALL_MS = 2500;

/**
 * Busca versoes novas em rfsp-Acc/ReCapt-releases ao abrir e a cada 6 h, baixa em segundo
 * plano e instala so quando o app esta ocioso: nunca no meio de gravacao ou transcricao.
 */
export function useUpdater(busy: boolean, notify: (kind: NoticeKind, text: string) => void) {
  const [ready, setReady] = useState<Update | null>(null);
  // Uma busca por vez; fica marcado depois que uma versao foi baixada.
  const pending = useRef(false);

  useEffect(() => {
    // Em modo dev o app nao e o instalado: atualizar substituiria o binario de teste.
    if (import.meta.env.DEV) return;
    const run = async () => {
      if (pending.current) return;
      pending.current = true;
      try {
        const update = await check();
        if (!update) {
          pending.current = false;
          return;
        }
        await update.download();
        setReady(update);
      } catch {
        // Sem internet ou servidor fora do ar: tenta de novo na proxima busca.
        pending.current = false;
      }
    };
    run();
    const timer = setInterval(run, CHECK_EVERY_MS);
    return () => clearInterval(timer);
  }, []);

  useEffect(() => {
    if (!ready || busy) return;
    notify("warning", `Instalando a versão ${ready.version}. O ReCapt vai reiniciar em instantes.`);
    const timer = setTimeout(() => {
      ready
        .install()
        .then(relaunch)
        .catch((e) => {
          notify("error", `Não foi possível instalar a atualização: ${e}`);
          setReady(null);
          pending.current = false;
        });
    }, NOTICE_BEFORE_INSTALL_MS);
    // Se uma gravacao comecar durante o aviso, a instalacao espera ela terminar.
    return () => clearTimeout(timer);
  }, [ready, busy, notify]);
}
