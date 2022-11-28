import { vars } from "@theme/contract.css";
import * as theme from "@theme/theme.css";
import { style } from "@vanilla-extract/css";

export const chatContainer = style({
  borderLeft: vars.border.solid,
  borderColor: vars.color.border.primary,
  height: `calc(100vh)`,
  background: vars.color.background.accentPrimary,
  display: "flex",
  flexDirection: "column",
  flex: 1,
  position: "relative",
  overflow: "hidden",
});

export const chatHeader = style({
  padding: vars.spacing.sm,
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  userSelect: "none",
  cursor: "default",
  position: "relative",
  zIndex: 9,
});

export const chatHeaderHeading = style([
  theme.textColor.primary,
  theme.display.flex,
  {
    fontSize: vars.text.md,
    fontWeight: 600,
    marginTop: vars.spacing.sm,
    alignItems: "center",
  },
]);

export const chatHeaderSubheading = style([
  theme.textColor.dim,
  {
    fontSize: vars.text.sm,
    fontWeight: 500,
    marginTop: vars.spacing.xs,
  },
]);

export const main = style({
  flex: 1,
});

export const footer = style({
  padding: vars.spacing.md,
  borderTop: vars.border.solid,
  borderColor: vars.color.border.primary,
  background: vars.color.background.accentPrimary,
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
  position: "relative",
  zIndex: 9,
});

export const planets = style({
  position: "absolute",
  transform: "scale(0.85) translate(-35%, -10%)",
  opacity: 0.35,
});
