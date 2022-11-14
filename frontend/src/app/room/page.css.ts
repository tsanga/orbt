import { style } from "@vanilla-extract/css";

export const main = style({
  display: "flex",
  width: "100vw",
});

export const leftSection = style({
  display: "flex",
  flexDirection: "column",
  flex: 1,
});

export const rightSection = style({
  minWidth: "450px",
  display: "flex",
});
