import { Show, createSignal } from "solid-js";
import { BENCHER_LOGO_URL, BENCHER_VERSION } from "../../../util/ext";
import ProjectSelect from "./ProjectSelect";
import { authUser } from "../../../util/auth";
import type { Params } from "astro";

export interface Props {
	params: Params;
}

const ConsoleNavbar = (props: Props) => {
	const user = authUser();

	const [burger, setBurger] = createSignal(false);
	const [dropdown, setDropdown] = createSignal(false);

	return (
		<nav class="navbar" role="navigation" aria-label="main navigation">
			<div class="navbar-brand">
				<a
					class="navbar-item"
					title="Console Home"
					href="/console/organizations"
				>
					<img
						src={BENCHER_LOGO_URL}
						width="152"
						height="28"
						alt="🐰 Bencher"
					/>
				</a>

				<button
					class={`navbar-burger ${burger() && "is-active"}`}
					aria-label="menu"
					aria-expanded="false"
					onClick={(_e) => setBurger(!burger())}
				>
					<span aria-hidden="true" />
					<span aria-hidden="true" />
					<span aria-hidden="true" />
				</button>
			</div>

			<div class={`navbar-menu ${burger() && "is-active"}`}>
				<div class="navbar-start">
					<a class="navbar-item" href="/docs">
						Docs
					</a>
					<a class="navbar-item" href="/perf">
						Public Projects
					</a>
					<Show
						when={props.params.organization || props.params.project}
						fallback={<></>}
					>
						<div class="navbar-item">
							<ProjectSelect params={props.params} user={user} />
						</div>
					</Show>
				</div>

				<div class="navbar-end">
					<div class="navbar-item">
						<div class="navbar-item">
							<a
								class="button is-outlined"
								href={`/console/users/${user?.user?.slug}/help`}
							>
								<span class="icon has-text-primary">
									<i class="fas fa-life-ring" aria-hidden="true" />
								</span>
								<span>Help</span>
							</a>
						</div>
						<div
							class={`navbar-item has-dropdown ${dropdown() && "is-active"}`}
						>
							<a class="navbar-link" onClick={(_e) => setDropdown(!dropdown())}>
								{user?.user?.name ? user?.user?.name : "Account"}
							</a>
							<div class="navbar-dropdown">
								<a
									class="navbar-item"
									href={`/console/users/${user?.user?.slug}/tokens`}
								>
									Tokens
								</a>
								<a
									class="navbar-item"
									href={`/console/users/${user?.user?.slug}/settings`}
								>
									Settings
								</a>
								<hr class="navbar-divider" />
								<div class="navbar-item">
									<a class="button is-light is-fullwidth" href="/auth/logout">
										Log out
									</a>
								</div>
								<hr class="navbar-divider" />
								<div class="navbar-item">BETA v{BENCHER_VERSION}</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</nav>
	);
};

export default ConsoleNavbar;