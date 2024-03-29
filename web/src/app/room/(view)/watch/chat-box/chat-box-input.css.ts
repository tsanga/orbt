import { style } from "@vanilla-extract/css";
import { vars } from "@theme/contract.css";
import * as theme from "@theme/theme.css";

export const inputContainer = style([
  theme.width.full,
  theme.display.flex,
  {
    borderRadius: vars.border.radius.md,
    background: vars.color.gradient.horizontal,
    position: "relative",
  },
]);

export const input = style([
  theme.background.accentPrimary,
  theme.input.primary,
  theme.input.lg,
  {
    flex: "1",
    paddingRight: "4.5em",
  },
]);

export const toolbarContainer = style([
  theme.display.flex,
  {
    position: "absolute",
    right: 0,
    height: "100%",
    alignItems: "center",
  },
]);

export const toolbar = style([
  theme.display.flex,
  {
    marginRight: vars.spacing.xs,
  },
]);
