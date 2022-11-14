import { createThemeContract } from "@vanilla-extract/css";

export const vars = createThemeContract({
  color: {
    background: {
      primary: "",
      secondary: "",
      accentPrimary: "",
      focus: "",
    },
    text: {
      primary: "",
      secondary: "",
      dim: ""
    },
    border: {
      primary: "",
      secondary: "",
    },
  },
  spacing: {
    xxs: "",
    xs: "",
    sm: "",
    md: "",
    lg: "",
    xl: "",
  },
  text: {
    xs: "",
    sm: "",
    md: "",
    lg: "",
  },
  border: {
    solid: "",
    radius: {
      sm: "",
      md: "",
      lg: "",
    },
  },
});
