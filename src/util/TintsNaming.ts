export type TintsNamingMode =
  | "5,10,20...90,95"
  | "50,100,200...900,950"
  | "0,10,20..."
  | "0,100,200..."
  | "manual";

export function getTintName({
  index,
  length,
  mode,
  existingName,
}: {
  index: number;
  length: number;
  mode: TintsNamingMode;
  existingName?: string;
}): string {
  if (mode === "manual" && existingName) {
    return existingName;
  }

  const usedMode: TintsNamingMode =
    mode === "manual" ? "50,100,200...900,950" : mode;

  if (usedMode === "5,10,20...90,95" || usedMode === "50,100,200...900,950") {
    const step = usedMode === "5,10,20...90,95" ? 10 : 100;
    const offset = usedMode === "5,10,20...90,95" ? 5 : 50;

    if (length === 1 || index === 0) return `${offset}`;

    if (index === length - 1)
      return `${index * step + (length > 11 ? offset : -offset)}`;

    return `${index * step}`;
  }

  if (usedMode === "0,10,20..." || usedMode === "0,100,200...") {
    const step = usedMode === "0,10,20..." ? 10 : 100;

    return `${index * step}`;
  }

  return "";
}

export const TINTS_NAMING_MODE: TintsNamingMode[] = [
  "50,100,200...900,950",
  "5,10,20...90,95",
  "0,10,20...",
  "0,100,200...",
  "manual",
];

export function isTintsNamingMode(value: string): value is TintsNamingMode {
  return [
    "50,100,200...900,950",
    "5,10,20...90,95",
    "0,10,20...",
    "0,100,200...",
    "manual",
  ].includes(value);
}
