import { style, styleVariants } from "@vanilla-extract/css";
import { vars } from "./contract.css";

export const background = styleVariants({
  primary: {
    background: vars.color.background.primary,
  },
  secondary: {
    background: vars.color.background.secondary,
  },
});

export const textColor = styleVariants({
  primary: {
    color: vars.color.text.primary,
  },
  secondary: {
    color: vars.color.text.secondary,
  },
  dim: {
    color: vars.color.text.dim,
  },
});

const buttonBase = style({
  background: "none",
  padding: "8px 12px",
  border: "none",
  borderRadius: vars.border.radius.md,
  display: "flex",
  alignItems: "center",
  cursor: "pointer",
  transition: "all 0.2s",
  transform: "translateZ(0)",
  userSelect: "none",
});

const buttonSmall = style({
  padding: "4px 6px",
  fontSize: vars.text.xs,
});

export const button = styleVariants({
  primary: [
    buttonBase,
    {
      background: vars.color.background.secondary,
      border: vars.border.solid,
      borderColor: vars.color.background.secondary,
      color: vars.color.text.secondary,
      fontWeight: 600,

      ":hover": {
        opacity: 0.8,
      },

      ":active": {
        background: "transparent",
        color: vars.color.text.primary,
        boxSizing: "border-box",
      },
    },
  ],
  link: [
    buttonBase,
    {
      ":hover": {
        opacity: 0.8,
      },
    },
  ],
  small: [buttonBase, buttonSmall],
});
