export default function cssTransformer(
  id: string,
  styles: {
    [key: string]: string;
  }
) {
  let out = `
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

  for (const [k, v] of Object.entries(styles).filter(([k]) =>
    k.startsWith(":")
  )) {
    out +=
      " " +
      `
      .iid_${id}${k} {
        ${Object.entries(v)
          .map(([key, value]) => `${key}: ${value}`)
          .join("~ ")}
        }
    `
        .replace(/\n/g, " ")
        .replace(/[ ]{2,}/g, " ")
        .trim();
  }

  return out;
}
