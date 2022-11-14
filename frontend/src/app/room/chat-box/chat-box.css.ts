import { vars, theme } from "@theme";
import { style } from "@vanilla-extract/css";

export const chatContainer = style({
  borderLeft: vars.border.solid,
  borderColor: vars.color.border.primary,
  height: `calc(100vh - (${vars.spacing.md})*2)`,
  background: vars.color.background.accentPrimary,
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
  {
    fontSize: vars.text.md,
    fontWeight: 600,
    marginTop: vars.spacing.xxs,
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
