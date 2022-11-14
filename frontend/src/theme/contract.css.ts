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
      placeholder: "",
      input: "",
      dim: "",
    },
    border: {
      primary: "",
      secondary: "",
    },
    gradient: {
      horizontal: "",
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
    __raw: "",
    radius: {
      sm: "",
      md: "",
      lg: "",
    },
  },
});
