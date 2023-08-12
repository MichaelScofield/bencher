import type { InitOutput } from "bencher_valid";
import { Resource, createEffect, createMemo } from "solid-js";
import { JsonAuthUser, PlanLevel } from "../../../../types/bencher";
import { useSearchParams } from "../../../../util/url";
import { validPlanLevel } from "../../../../util/valid";
import { PLAN_PARAM } from "../../../auth/auth";
import Pricing from "./Pricing";
import PaymentCard from "./PaymentCard";
import { BENCHER_API_URL } from "../../../../util/ext";
import type { Params } from "astro";

interface Props {
	params: Params;
	bencher_valid: Resource<InitOutput>;
	user: JsonAuthUser;
	handleRefresh: () => void;
}

const Billing = (props: Props) => {
	const [searchParams, setSearchParams] = useSearchParams();

	const setPlanLevel = (plan_level: PlanLevel) => {
		setSearchParams({ [PLAN_PARAM]: plan_level });
	};
	const plan = createMemo(() => searchParams[PLAN_PARAM] as PlanLevel);

	createEffect(() => {
		if (!props.bencher_valid()) {
			return;
		}
		if (!validPlanLevel(searchParams[PLAN_PARAM])) {
			setPlanLevel(PlanLevel.Free);
		}
	});

	return (
		<div class="columns is-centered">
			<div class="column">
				<Pricing
					plan={plan()}
					freeText="Stick with Free"
					handleFree={() => setPlanLevel(PlanLevel.Free)}
					teamText="Go with Team"
					handleTeam={() => setPlanLevel(PlanLevel.Team)}
					enterpriseText="Go with Enterprise"
					handleEnterprise={() => setPlanLevel(PlanLevel.Enterprise)}
				/>
				<br />
				{plan() !== PlanLevel.Free && (
					<PaymentCard
						params={props.params}
						bencher_valid={props.bencher_valid}
						user={props.user}
						url={`${BENCHER_API_URL()}/v0/organizations/${
							props.params.organization
						}/plan`}
						plan={plan}
						handleRefresh={props.handleRefresh}
					/>
				)}
			</div>
		</div>
	);
};

export default Billing;