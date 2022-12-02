import * as styles from "./modal.css";
import CloseIcon from "@assets/svg/icon/close.svg";

type Props = {
  children: React.ReactNode;
  onClose?: () => void;
};

export const Modal = ({ children, onClose }: Props) => {
  return (
    <div className={styles.container}>
      <div className={styles.backdrop}></div>
      <div className={styles.modal}>
        <div className={styles.closeButton} onClick={onClose}>
          <CloseIcon />
        </div>
        {children}
      </div>
    </div>
  );
};
