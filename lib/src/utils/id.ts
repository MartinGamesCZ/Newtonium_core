import { randomUUID } from "crypto";

// Function to generate a random id (32 characters)
export function randomId() {
  return randomUUID().replaceAll("-", "");
}
