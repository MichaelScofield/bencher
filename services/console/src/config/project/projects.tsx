import type { Params } from "astro";
import FieldKind from "../../components/field/kind";
import { isAllowedProjectEdit } from "../../util/auth";
import { BENCHER_API_URL } from "../../util/ext";
import { validNonEmpty, validSlug, validOptionUrl } from "../../util/valid";
import {
	ActionButton,
	Button,
	Card,
	Display,
	Operation,
	PerfTab,
	Row,
} from "../types";
import { addPath, parentPath } from "../util";
import type { JsonProject } from "../../types/bencher";

const PROJECT_FIELDS = {
	name: {
		type: "text",
		placeholder: "Project Name",
		icon: "fas fa-project-diagram",
		help: "Must be non-empty string",
		validate: validNonEmpty,
	},
	slug: {
		type: "text",
		placeholder: "Project Slug",
		icon: "fas fa-exclamation-triangle",
		help: "Must be a valid slug",
		validate: validSlug,
	},
	url: {
		type: "text",
		placeholder: "https://www.example.com",
		icon: "fas fa-link",
		help: "Must be a valid URL",
		validate: validOptionUrl,
	},
	visibility: {
		icon: "fas fa-eye",
	},
};

const VISIBILITY_VALUE = {
	selected: "public",
	options: [
		{
			value: "public",
			option: "Public",
		},
		{
			value: "private",
			option: "Private",
		},
	],
};

const projectsConfig = {
	[Operation.LIST]: {
		operation: Operation.LIST,
		header: {
			title: "Projects",
			buttons: [
				{
					kind: Button.ADD,
					title: "Project",
					path: addPath,
				},
				{ kind: Button.REFRESH },
			],
		},
		table: {
			url: (params: Params) =>
				`${BENCHER_API_URL()}/v0/organizations/${
					params?.organization
				}/projects`,
			add: {
				prefix: (
					<div>
						<h4>🐰 Create your first project!</h4>
						<p>
							It's easy to create a project.
							<br />
							Tap below to get started.
						</p>
					</div>
				),
				path: addPath,
				text: "Add a Project",
			},
			row: {
				key: "name",
				items: [
					{
						kind: Row.TEXT,
						key: "slug",
					},
					{},
					{
						kind: Row.TEXT,
						key: "visibility",
					},
					{},
				],
				button: {
					text: "Select",
					path: (_pathname: string, datum: JsonProject) =>
						`/console/projects/${datum?.slug}/perf`,
				},
			},
			name: "projects",
		},
	},
	[Operation.ADD]: {
		operation: Operation.ADD,
		header: {
			title: "Add Project",
			path: parentPath,
			path_to: "Projects",
		},
		form: {
			url: (params: Params) =>
				`${BENCHER_API_URL()}/v0/organizations/${params.organization}/projects`,
			fields: [
				{
					kind: FieldKind.INPUT,
					label: "Name",
					key: "name",
					value: "",
					valid: null,
					validate: true,
					config: PROJECT_FIELDS.name,
				},
				{
					kind: FieldKind.INPUT,
					label: "URL",
					key: "url",
					value: "",
					valid: true,
					validate: true,
					nullable: true,
					config: PROJECT_FIELDS.url,
				},
				{
					kind: FieldKind.SELECT,
					label: "Visibility",
					key: "visibility",
					value: VISIBILITY_VALUE,
					validate: false,
					config: PROJECT_FIELDS.visibility,
				},
			],
			path: parentPath,
		},
	},
	[Operation.VIEW]: {
		operation: Operation.VIEW,
		header: {
			key: "name",
			path: parentPath,
			path_to: "Projects",
			buttons: [{ kind: Button.REFRESH }],
		},
		deck: {
			url: (params: Params) =>
				`${BENCHER_API_URL()}/v0/projects/${params?.project}`,
			cards: [
				{
					kind: Card.FIELD,
					label: "Project Name",
					key: "name",
					display: Display.RAW,
					is_allowed: isAllowedProjectEdit,
					field: {
						kind: FieldKind.INPUT,
						label: "Name",
						key: "name",
						value: "",
						valid: null,
						validate: true,
						config: PROJECT_FIELDS.name,
					},
				},
				{
					kind: Card.FIELD,
					label: "Project Slug",
					key: "slug",
					display: Display.RAW,
					is_allowed: isAllowedProjectEdit,
					field: {
						kind: FieldKind.INPUT,
						label: "Slug",
						key: "slug",
						value: "",
						valid: null,
						validate: true,
						config: PROJECT_FIELDS.slug,
					},
					path: (_params: Params, data: JsonProject) =>
						`/console/projects/${data?.slug}/settings`,
				},
				{
					kind: Card.FIELD,
					label: "Project UUID",
					key: "uuid",
					display: Display.RAW,
				},
				{
					kind: Card.FIELD,
					label: "Project URL",
					key: "url",
					display: Display.RAW,
					is_allowed: isAllowedProjectEdit,
					field: {
						kind: FieldKind.INPUT,
						label: "URL",
						key: "url",
						value: "",
						valid: null,
						validate: true,
						nullable: true,
						config: PROJECT_FIELDS.url,
					},
				},
				{
					kind: Card.FIELD,
					label: "Visibility",
					key: "visibility",
					display: Display.SELECT,
					is_allowed: isAllowedProjectEdit,
					field: {
						kind: FieldKind.SELECT,
						key: "visibility",
						value: VISIBILITY_VALUE,
						validate: false,
						config: PROJECT_FIELDS.visibility,
					},
				},
			],
			buttons: [
				{
					kind: ActionButton.DELETE,
					subtitle: null,
					path: (_pathname: string, data: JsonProject) =>
						`/console/organizations/${data.organization}/projects`,
				},
			],
		},
	},
	[Operation.PERF]: {
		operation: Operation.PERF,
		header: {
			url: (project_slug: string) =>
				`${BENCHER_API_URL()}/v0/projects/${project_slug}/perf/img`,
		},
		plot: {
			project_url: (project_slug: string) =>
				`${BENCHER_API_URL()}/v0/projects/${project_slug}`,
			perf_url: (project_slug: string) =>
				`${BENCHER_API_URL()}/v0/projects/${project_slug}/perf`,
			tab_url: (project_slug: string, tab: PerfTab) =>
				`${BENCHER_API_URL()}/v0/projects/${project_slug}/${tab}`,
			metric_kinds_url: (project_slug: string) =>
				`${BENCHER_API_URL()}/v0/projects/${project_slug}/metric-kinds`,
		},
	},
};

export default projectsConfig;