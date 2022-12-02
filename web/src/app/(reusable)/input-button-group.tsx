"use client";

export type Type = "dark";
export type ButtonType = "gradient";
export type Size = "small" | "medium" | "large";

export type Props = {
  id: string;
  type?: Type;
  label?: string;
  buttonType?: ButtonType;
  buttonText?: React.ReactNode;
  placeholder?: string;
  onChange?: (value: string) => void;
  onSubmit?: () => void;
  isSubmitting?: boolean;
};

export default function InputButtonGroup({
  id,
  type,
  label,
  buttonType,
  buttonText,
  placeholder,
  onChange,
  onSubmit,
  isSubmitting,
}: Props) {
  return (
    <div>
      {!!label && <label htmlFor={id}>{label}</label>}
      <div>
        <input
          placeholder={placeholder}
          onChange={(event) => onChange?.(event.target.value)}
        ></input>
        <button onClick={() => onSubmit?.()} id={id}>
          {buttonText || "Send"}
        </button>
      </div>
    </div>
  );
}
