import { createTheme } from "@vanilla-extract/css";
import { vars } from "./contract.css";

export const darkThemeClass = createTheme(vars, {
  color: {
    background: {
      primary: "black",
      secondary: "white",
      accentPrimary: "#0A0A0C",
      focus: "#0C0C0E",
    },
    text: {
      primary: "white",
      secondary: "black",
      dim: "#2C2C36",
    },
    border: {
      primary: "#2C2C36",
      secondary: "",
    },
  },
  spacing: {
    xxs: "0.25em",
    xs: "0.5em",
    sm: "1em",
    md: "1.5em",
    lg: "2em",
    xl: "2.5em",
  },
  text: {
    xs: "0.5em",
    sm: "0.8em",
    md: "1em",
    lg: "1.5em",
  },
  border: {
    solid: "1.5px solid",
    radius: {
      sm: "3px",
      md: "6px",
      lg: "9px",
    },
  },
});
