import * as styles from "./page.css";
import RoomTopBar from "./top-bar/room-top-bar";
import RoomChatBox from "./chat-box/chat-box";

export default function RoomPage() {
  return (
    <main className={styles.main}>
      <section className={styles.leftSection}>
        <RoomTopBar></RoomTopBar>
      </section>
      <section className={styles.rightSection}>
        <RoomChatBox subheading={"Hello"}></RoomChatBox>
      </section>
    </main>
  );
}
