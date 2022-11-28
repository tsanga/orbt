import { style, styleVariants } from "@vanilla-extract/css";
import { vars } from "@theme/contract.css";

export const header = style({
  padding: vars.spacing.xxl,
  display: "flex",
  justifyContent: "center",
});

export const container = style({
  width: "100vw",
  height: "100vh",
  position: "fixed",
  background: vars.color.background.accentPrimary,
  display: "flex",
  flexDirection: "column",
});

export const overlay = styleVariants({
  outer: {
    position: "absolute",
    width: "100vw",
    height: "100vh",
  },
  inner: {
    transform: "scale(1.4)",
  },
});

export const main = style({
  position: "relative",
  zIndex: 1,
  width: "100vw",
  display: "flex",
  justifyContent: "center",
});

export const footer = style({});
