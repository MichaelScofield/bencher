import { createSignal, type Accessor, Show } from "solid-js";
import type { JsonAuthUser } from "../../../../../types/bencher";
import ViewCard from "./ViewCard";
import UpdateCard from "./UpdateCard";
import type CardConfig from "./CardConfig";
import type { Params } from "astro";

export interface Props {
	params: Params;
	user: JsonAuthUser;
	url: Accessor<string>;
	card: CardConfig;
	value: boolean | string;
	handleRefresh: () => void;
	handleLoopback: (pathname: null | string) => void;
}

const FieldCard = (props: Props) => {
	const [update, setUpdate] = createSignal(false);

	const toggleUpdate = () => {
		setUpdate(!update());
	};

	return (
		<Show
			when={update()}
			fallback={
				<ViewCard
					params={props.params}
					card={props.card}
					value={props.value}
					// refresh={props.refresh}
					toggleUpdate={toggleUpdate}
				/>
			}
		>
			<UpdateCard
				params={props.params}
				user={props.user}
				url={props.url}
				card={props.card}
				value={props.value}
				toggleUpdate={toggleUpdate}
				handleRefresh={props.handleRefresh}
				handleLoopback={props.handleLoopback}
			/>
		</Show>
	);
};

export default FieldCard;