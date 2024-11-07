export default function cssTransformer(
  id: string,
  styles: {
    [key: string]: string;
  }
) {
  const out = `
    .iid_${id} {
      ${Object.entries(styles)
        .filter(([, v]) => typeof v === "string")
        .map(([key, value]) => `${key}: ${value}`)
        .join("~ ")}
    }
  `
    .replace(/\n/g, " ")
    .replace(/[ ]{2,}/g, " ")
    .trim();

  return out;
}
