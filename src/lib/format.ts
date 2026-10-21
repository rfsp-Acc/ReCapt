// As pastas usam AAAA-MM-DD para ordenar cronologicamente; na interface a data aparece como DD-MM-AAAA.
const FOLDER_DATE = /^(\d{4})-(\d{2})-(\d{2})(?:_(.*))?$/;

export function formatDate(date: Date): string {
  const dd = String(date.getDate()).padStart(2, "0");
  const mm = String(date.getMonth() + 1).padStart(2, "0");
  return `${dd}-${mm}-${date.getFullYear()}`;
}

/** "2026-10-20_15h40" -> "20-10-2026 15h40"; "2026-10-20_retro-sprint" -> "20-10-2026 retro-sprint". */
export function formatSessionName(name: string): string {
  const m = FOLDER_DATE.exec(name);
  if (!m) return name;
  const [, yyyy, mm, dd, rest] = m;
  return rest ? `${dd}-${mm}-${yyyy} ${rest}` : `${dd}-${mm}-${yyyy}`;
}

/** Data e titulo de uma sessao: o titulo e o horario (15h40) ou o nome dado depois. */
export function parseSessionName(name: string): { date: Date | null; title: string } {
  const m = FOLDER_DATE.exec(name);
  if (!m) return { date: null, title: name };
  const [, yyyy, mm, dd, rest] = m;
  return { date: new Date(Number(yyyy), Number(mm) - 1, Number(dd)), title: rest ?? "" };
}

/** "Hoje", "Ontem" ou DD-MM-AAAA. */
export function dayLabel(date: Date | null, today = new Date()): string {
  if (!date) return "Outras";
  const start = (d: Date) => new Date(d.getFullYear(), d.getMonth(), d.getDate()).getTime();
  const days = Math.round((start(today) - start(date)) / 86_400_000);
  if (days === 0) return "Hoje";
  if (days === 1) return "Ontem";
  return formatDate(date);
}

/** "45 s", "3 min 20 s", "32 min", "1 h 02 min": segundos so importam em gravacoes curtas. */
export function formatDuration(ms: number): string {
  const total = Math.round(ms / 1000);
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  if (h > 0) return `${h} h ${String(m).padStart(2, "0")} min`;
  if (m >= 10) return `${m} min`;
  if (m > 0) return `${m} min ${String(s).padStart(2, "0")} s`;
  return `${s} s`;
}

/** Cronometro: "04:12" ou "1:04:12". */
export function formatClock(totalSeconds: number): string {
  const h = Math.floor(totalSeconds / 3600);
  const m = Math.floor((totalSeconds % 3600) / 60);
  const s = totalSeconds % 60;
  const mmss = `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
  return h > 0 ? `${h}:${mmss}` : mmss;
}
