import { style, styleVariants } from "@vanilla-extract/css";
import { vars } from "@theme/contract.css";
import * as theme from "@theme/theme.css";

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
  flexDirection: "column",
  alignItems: "center",
  flex: 1,
});

export const footer = style({
  padding: vars.spacing.xxl,
  paddingBottom: vars.spacing.xl,
  display: "flex",
  justifyContent: "center",
  position: "relative",
  zIndex: 1,
  flexDirection: "column",
  alignItems: "center",
});

export const socialList = style({
  display: "flex",
  alignItems: "center",
});

export const socialItem = style({
  margin: vars.spacing.xs,
  transform: "translteZ(0)",
  transition: "opacity 0.1s",
  ":hover": {
    opacity: 0.7,
  },
});

export const orgLogo = style([
  theme.textColor.dim,
  {
    transform: "translteZ(0)",
    transition: "opacity 0.1s",
    ":hover": {
      opacity: 0.7,
    },
  },
]);
