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
