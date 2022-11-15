import { vars, theme } from "@theme";
import { style } from "@vanilla-extract/css";

export const chatContainer = style({
  borderLeft: vars.border.solid,
  borderColor: vars.color.border.primary,
  height: `calc(100vh)`,
  background: vars.color.background.accentPrimary,
  display: "flex",
  flexDirection: "column",
  flex: 1,
});

export const chatHeader = style({
  padding: vars.spacing.sm,
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  userSelect: "none",
  cursor: "default",
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
  minHeight: 70,
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
});
