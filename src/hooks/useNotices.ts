import { useCallback, useEffect, useRef, useState } from "react";

export type NoticeKind = "error" | "warning" | "success";

export interface Notice {
  id: number;
  kind: NoticeKind;
  text: string;
}

/** Avisos empilhados no canto da tela; sucessos somem sozinhos, erros ficam ate fechar. */
export function useNotices() {
  const [notices, setNotices] = useState<Notice[]>([]);
  const nextId = useRef(1);

  const dismiss = useCallback((id: number) => {
    setNotices((all) => all.filter((n) => n.id !== id));
  }, []);

  const push = useCallback(
    (kind: NoticeKind, text: string) => {
      const id = nextId.current++;
      // O mesmo texto nao se repete na pilha.
      setNotices((all) => (all.some((n) => n.text === text) ? all : [...all, { id, kind, text }]));
      if (kind === "success") setTimeout(() => dismiss(id), 3500);
    },
    [dismiss],
  );

  return { notices, push, dismiss };
}

/** Publica como aviso de erro cada novo erro de um hook. */
export function useErrorNotice(error: string | null, push: (kind: NoticeKind, text: string) => void) {
  useEffect(() => {
    if (error) push("error", error);
  }, [error, push]);
}
