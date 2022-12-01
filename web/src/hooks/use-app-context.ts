import { AppContext, AppDispatchContext, State, Action } from "@/context";
import { useContext, Dispatch } from "react";

export default function useAppContext(): [State, Dispatch<Action>] {
  const state = useContext(AppContext);
  const dispatch = useContext(AppDispatchContext);

  return [state!!, dispatch!!];
}
