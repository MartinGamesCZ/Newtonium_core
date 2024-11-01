import { randomUUID } from "crypto";

export function randomId() {
  return randomUUID().replaceAll("-", "");
}
