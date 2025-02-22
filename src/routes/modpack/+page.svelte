<script lang="ts">
	import InputField from '$lib/components/InputField.svelte';
	import FormField from '$lib/components/FormField.svelte';
	import Checkbox from '$lib/components/Checkbox.svelte';
	import BigButton from '$lib/components/BigButton.svelte';
	import PathField from '$lib/components/PathField.svelte';
	import Markdown from '$lib/components/Markdown.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import Link from '$lib/components/Link.svelte';
	import ApiKeyPopup, { apiKeyPopupOpen } from '$lib/prefs/ApiKeyPopup.svelte';

	import { invokeCommand } from '$lib/invoke';
	import type { ModpackArgs, PackageCategory } from '$lib/models';
	import { activeProfile, activeGame, categories } from '$lib/stores';
	import { open } from '@tauri-apps/plugin-dialog';
	import { onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import Icon from '@iconify/svelte';

	import { Button, Dialog, Select } from 'bits-ui';
	import Popup from '$lib/components/Popup.svelte';
	import Checklist from '$lib/components/Checklist.svelte';
	import ResizableInputField from '$lib/components/ResizableInputField.svelte';

	const URL_PATTERN =
		'[Hh][Tt][Tt][Pp][Ss]?://(?:(?:[a-zA-Z\u00a1-\uffff0-9]+-?)*[a-zA-Z\u00a1-\uffff0-9]+)(?:.(?:[a-zA-Z\u00a1-\uffff0-9]+-?)*[a-zA-Z\u00a1-\uffff0-9]+)*(?:.(?:[a-zA-Z\u00a1-\uffff]{2,}))(?::d{2,5})?(?:/[^s]*)?';

	let name: string;
	let author: string;
	let selectedCategories: PackageCategory[] = [];
	let nsfw: boolean;
	let description: string;
	let readme: string;
	let changelog: string;
	let versionNumber: string;
	let iconPath: string;
	let websiteUrl: string;
	let includeDisabled: boolean;
	let includeFiles = new Map<string, boolean>();

	let donePopupOpen = false;
	let loading: string | null = null;

	let includedFileCount = 0;

	$: {
		$activeProfile;
		refresh();
	}

	// some communities don't have a specific modpack category
	$: modpackCategoryExists = $categories.some((category) => category.slug === 'modpacks');

	// make sure the modpacks category is always selected if it exists
	$: if (
		modpackCategoryExists &&
		selectedCategories &&
		!selectedCategories.some((category) => category?.slug === 'modpacks')
	) {
		selectedCategories = [
			$categories.find((category) => category.slug === 'modpacks')!,
			...selectedCategories
		];
	}

	$: includedFileCount = countIncludedFiles(includeFiles);

	function countIncludedFiles(includeFiles?: Map<string, boolean>) {
		if (!includeFiles) return 0;

		let count = 0;
		for (let enabled of includeFiles.values()) {
			if (enabled) count++;
		}
		return count;
	}

	async function refresh() {
		loading = '加载';

		let args = await invokeCommand<ModpackArgs>('get_pack_args');

		name = args.name;
		author = args.author;
		nsfw = args.nsfw;
		description = args.description;
		selectedCategories = args.categories.map(
			(selected) => $categories.find((category) => category.slug === selected)!
		);
		changelog = args.changelog;
		readme = args.readme;
		versionNumber = args.versionNumber;
		iconPath = args.iconPath;
		websiteUrl = args.websiteUrl;
		includeDisabled = args.includeDisabled;
		includeFiles = new Map(Object.entries(args.includeFileMap));

		loading = null;
	}

	async function browseIcon() {
		let path = await open({
			defaultPath: iconPath.length > 0 ? iconPath : undefined,
			title: '选择 modpack 图标',
			filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif'] }]
		});

		if (path === null) return;
		iconPath = path;
		saveArgs();
	}

	async function generateChangelog(all: boolean) {
		changelog = await invokeCommand('generate_changelog', { args: args(), all });
		saveArgs();
	}

	async function exportToFile() {
		let dir = await open({
			title: '选择要保存 modpack 的文件夹',
			defaultPath: `${name}.zip`,
			directory: true
		});

		if (!dir) return;

		loading = '正在将 modpack 导出到文件...';
		try {
			await invokeCommand('export_pack', { args: args(), dir });
		} finally {
			loading = null;
		}
	}

	async function uploadToThunderstore() {
		let hasToken = await invokeCommand('has_thunderstore_token');

		if (!hasToken) {
			$apiKeyPopupOpen = true;

			await new Promise<void>((resolve) => {
				const interval = setInterval(() => {
					if (!$apiKeyPopupOpen) {
						clearInterval(interval);
						resolve();
					}
				}, 100);

				return () => clearInterval(interval);
			});

			hasToken = await invokeCommand('has_thunderstore_token');

			if (!hasToken) return;
		}

		loading = '正在将 modpack 上传到 Thunderstore...';
		try {
			await invokeCommand('upload_pack', { args: args() });
			donePopupOpen = true;
		} finally {
			loading = null;
		}
	}

	function saveArgs() {
		// wait a tick to ensure the variables are updated
		setTimeout(() => {
			invokeCommand('set_pack_args', { args: args() });
		});
	}

	function args(): ModpackArgs {
		return {
			name,
			description,
			author,
			nsfw,
			readme,
			changelog,
			versionNumber,
			iconPath,
			websiteUrl,
			includeDisabled,
			includeFileMap: includeFiles,
			categories: selectedCategories.map(({ slug }) => slug)
		};
	}
</script>

<div class="relative mx-auto flex w-full max-w-4xl flex-col gap-1.5 overflow-y-auto px-6 py-4">
	{#if loading}
		<div
			class="fixed inset-0 flex items-center justify-center bg-black/40 text-lg text-slate-200"
			transition:fade={{ duration: 50 }}
		>
			<Icon icon="mdi:loading" class="mr-4 animate-spin" />
			{loading}
		</div>
	{/if}

	<FormField
		label="Name"
		description="整合包的名称，如 Thunderstore 上所示。请确保这在更新之间保持一致。不能包含空格或连字符。"
		required={true}
	>
		<InputField
			on:change={saveArgs}
			bind:value={name}
			placeholder="输入名称..."
			required={true}
			pattern="^[a-zA-Z0-9_]+$"
			class="w-full"
		/>
	</FormField>

	<FormField
		label="作者"
		description="整合包的作者，应该是您的 Thunderstore 团队的名称."
		required={true}
	>
		<InputField
			on:change={saveArgs}
			bind:value={author}
			placeholder="输入 作者..."
			required={true}
			class="w-full"
		/>
	</FormField>

	<FormField label="描述" description="整合包的简短描述。" required={true}>
		<InputField
			on:change={saveArgs}
			bind:value={description}
			placeholder="输入描述..."
			required={true}
			maxlength={250}
			class="w-full"
		/>
	</FormField>

	<FormField
		label="类别"
		description="整合包所属的类别。“整合包”始终包含在内."
	>
		<Dropdown
			avoidCollisions={false}
			items={$categories}
			bind:selected={selectedCategories}
			onSelectedChange={saveArgs}
			multiple={true}
			getLabel={(category) => category.name}
		>
			<Select.Trigger
				let:open
				slot="trigger"
				class="flex w-full items-center overflow-hidden rounded-lg border border-transparent bg-slate-900 py-1 pr-3 pl-1 hover:border-slate-500"
			>
				{#if selectedCategories.length === 0}
					<span class="truncate pl-2 text-slate-400">Select categories...</span>
				{:else}
					<div class="flex flex-wrap gap-1">
						{#each selectedCategories as category}
							<div class="rounded-md bg-slate-800 py-1 pr-1 pl-3 text-sm text-slate-200">
								<span class="truncate overflow-hidden">{category.name}</span>

								<Button.Root
									class="ml-1 rounded-md px-1.5 hover:bg-slate-700"
									on:click={(evt) => {
										evt.stopPropagation();
										selectedCategories = selectedCategories.filter((c) => c !== category);
									}}
								>
									x
								</Button.Root>
							</div>
						{/each}
					</div>
				{/if}
				<Icon
					class="ml-auto shrink-0 origin-center transform text-xl text-slate-400 transition-all
                duration-100 ease-out {open ? 'rotate-180' : 'rotate-0'}"
					icon="mdi:chevron-down"
				/>
			</Select.Trigger>
		</Dropdown>
	</FormField>

	<FormField
		label="版本"
		description="整合包的版本号，格式为 X.Y.Z.
			           不能使用相同的版本号发布两次。"
		required={true}
	>
		<InputField
			on:change={saveArgs}
			bind:value={versionNumber}
			placeholder="输入版本号..."
			required={true}
			pattern="^\d+\.\d+\.\d+$"
			class="w-full"
		/>
	</FormField>

	<FormField label="网站" description="您选择的网站的 URL 自选。">
		<InputField
			on:change={saveArgs}
			bind:value={websiteUrl}
			placeholder="输入网站 URL..."
			pattern={URL_PATTERN}
			class="w-full"
		/>
	</FormField>

	<FormField
		label="图标"
		description="整合包图标的路径。这会自动调整为 256x256 像素,
               建议为方形图像，以避免拉伸或挤压."
		required={true}
	>
		<PathField icon="mdi:file-image" on:click={browseIcon} value={iconPath} />
	</FormField>

	<FormField
		label="自述文件"
		description="支持 Markdown 格式的 modpack 的较长描述
                 (类似于 Discord 消息)."
		required={true}
	>
		<ResizableInputField
			on:change={saveArgs}
			bind:value={readme}
			placeholder="输入自述文件..."
			mono={true}
		/>

		<details class="mt-1">
			<summary class="cursor-pointer text-sm text-slate-300">Preview</summary>
			<Markdown class="mt-1 px-4" source={readme} />
			<div class="mt-4 h-[2px] bg-slate-500" />
		</details>
	</FormField>

	<FormField
		label="更改日志"
		description="modpack 中的更改列表也支持 markdown 格式。留空可省略。"
	>
		<ResizableInputField
			on:change={saveArgs}
			bind:value={changelog}
			placeholder="输入 更改日志..."
			mono={true}
		/>

		<BigButton color="slate" on:click={() => generateChangelog(false)}
			>Generate for {versionNumber}</BigButton
		>
		<BigButton color="slate" on:click={() => generateChangelog(true)}>Generate all</BigButton>

		<details class="mt-1">
			<summary class="cursor-pointer text-sm text-slate-300">Preview</summary>
			<Markdown class="mt-1 px-4" source={changelog} />
			<div class="mt-4 h-[2px] bg-slate-500" />
		</details>
	</FormField>

	<FormField
		label="导入文件 ({includedFileCount}/{includeFiles?.size})"
		description="选择要包含在 modpack 中的配置文件。"
	>
		<details>
			{#if includeFiles}
				<summary class="cursor-pointer text-sm text-slate-300">Show list</summary>
				<Checklist
					class="mt-1"
					title="导入全部"
					items={Array.from(includeFiles.keys()).sort()}
					getLabel={(item) => item}
					get={(item) => includeFiles.get(item) ?? false}
					set={(item, _, value) => {
						includeFiles.set(item, value);
						includeFiles = includeFiles;
					}}
				/>
			{/if}
		</details>
	</FormField>

	<div class="mt-1 flex items-center text-lg font-medium text-slate-200">
		<span class="max-w-96 grow">包含 NSFW 内容</span>

		<Checkbox onValueChanged={saveArgs} bind:value={nsfw} />
	</div>

	<div class="flex items-center text-lg font-medium text-slate-200">
		<span class="max-w-96 grow">包含已禁用的 Mod</span>

		<Checkbox onValueChanged={saveArgs} bind:value={includeDisabled} />
	</div>

	<div class="mt-3 flex justify-end gap-2">
		<BigButton color="slate" on:click={exportToFile}>导出到文件</BigButton>
		<BigButton color="accent" on:click={uploadToThunderstore}>在 Thunderstore 上发布</BigButton>
	</div>
</div>

<ApiKeyPopup />

<Popup bind:open={donePopupOpen} title="整合包上传完成">
	<Dialog.Description class="text-slate-300">
		{name}
		{versionNumber}已成功发布到 Thunderstore!
		<Link href="https://thunderstore.io/c/{$activeGame?.slug}/p/{author}/{name}"
			>单击此处在网站上查看其页面</Link
		>.
	</Dialog.Description>

	<div class="mt-2 text-sm text-slate-400">
	这些更改可能需要长达一个小时才能显示在 Gale 和其他 Mod 管理器中。
		<br />
	要发布新的更新，请递增版本号并再次发布 modpack。
	</div>
</Popup>
