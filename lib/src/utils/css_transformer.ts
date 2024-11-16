// Function to convert styles object into css string
export default function cssTransformer(
  id: string,
  styles: {
    [key: string]: string;
  }
) {
  // Convert styles object into css class and prettify it (remove new lines and multiple spaces)
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

  // Convert pseudo-classes and modifiers into css classes
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
