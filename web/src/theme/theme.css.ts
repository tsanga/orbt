import {
  style,
  styleVariants,
  ComplexStyleRule,
  keyframes,
} from "@vanilla-extract/css";
import { vars } from "./contract.css";

export const display = styleVariants(
  ["flex", "display", "block", "inline"].reduce(
    (x: { [key: string]: ComplexStyleRule }, c: string) => (
      (x[c] = { display: c }), x
    ),
    {}
  )
);

export const width = styleVariants({
  full: {
    width: "100%",
  },
});

type BackgroundColor = keyof typeof vars.color.background;
export const background = styleVariants(
  Object.keys(vars.color.background)
    .map((x) => x as BackgroundColor)
    .reduce(
      (x: { [key: string]: ComplexStyleRule }, c: BackgroundColor) => (
        (x[c] = { background: vars.color.background[c] }), x
      ),
      {}
    )
);

type TextColor = keyof typeof vars.color.text;
export const textColor = styleVariants(
  Object.keys(vars.color.text)
    .map((x) => x as TextColor)
    .reduce(
      (x: { [key: string]: ComplexStyleRule }, c: TextColor) => (
        (x[c] = { color: vars.color.text[c] }), x
      ),
      {}
    )
);

type TextSize = keyof typeof vars.text;
export const text = styleVariants(
  Object.keys(vars.text)
    .map((x) => x as TextSize)
    .reduce(
      (x: { [key: string]: ComplexStyleRule }, c: TextSize) => (
        (x[c] = { fontSize: vars.text[c] }), x
      ),
      {}
    )
);

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

const buttonExtraSmall = style({
  padding: "2px 3px",
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
  sm: [buttonBase, buttonSmall],
  xs: [buttonBase, buttonExtraSmall],
});

const inputBase = style({
  outline: "none",
  color: vars.color.text.input,
  padding: "12px 12px",
  border: vars.border.solid,
  borderColor: vars.color.border.primary,
  borderRadius: vars.border.radius.md,
  "::placeholder": {
    color: vars.color.text.placeholder,
  },
});

export const input = styleVariants({
  primary: [inputBase],
  lg: {
    fontSize: vars.text.md,
  },
});

const spin = keyframes({
  "0%": { transform: "rotate(0deg)" },
  "100%": { transform: "rotate(360deg)" },
});

const pulse = keyframes({
  "0%, 100%": {
    opacity: 1,
  },
  "50%": {
    opacity: 0.5,
  },
});

const pulseLow = keyframes({
  "0%, 100%": {
    opacity: 0.2,
  },
  "50%": {
    opacity: 0.1,
  },
});

export const animation = {
  spin,
  pulse,
};

export const animate = styleVariants({
  spin: {
    animation: spin,
    animationDuration: "3s",
  },
  pulse: {
    animation: `${pulse} 2s cubic-bezier(0.4, 0, 0.6, 1) infinite`,
  },
  pulseLow: {
    animation: `${pulseLow} 2s cubic-bezier(0.4, 0, 0.6, 1) infinite`,
  },
});
