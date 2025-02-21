<script lang="ts">
	import Popup from '$lib/components/Popup.svelte';
	import TabsMenu from '$lib/components/TabsMenu.svelte';

	import { Tabs } from 'bits-ui';

	import { invokeCommand } from '$lib/invoke';
	import type { ImportData } from '$lib/models';
	import Icon from '@iconify/svelte';
	import { readText } from '@tauri-apps/plugin-clipboard-manager';
	import { confirm } from '@tauri-apps/plugin-dialog';
	import InputField from '$lib/components/InputField.svelte';
	import { profiles, refreshProfiles } from '$lib/stores';
	import BigButton from '$lib/components/BigButton.svelte';
	import Label from '$lib/components/Label.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import ModCardList from '$lib/modlist/ModCardList.svelte';
	import Tooltip from '$lib/components/Tooltip.svelte';
	import Checkbox from '$lib/components/Checkbox.svelte';
	import Info from '$lib/components/Info.svelte';

	export let open: boolean;
	export let data: ImportData | null;

	let key: string;
	let name: string;
	let loading: boolean;
	let importAll: boolean;
	let mode: 'new' | 'overwrite' = 'new';

	$: if (open) {
		getKeyFromClipboard();
	}

	$: if (mode === 'overwrite' && isAvailable(name)) {
		name = profiles[0].name;
	}

	$: nameAvailable = mode === 'overwrite' || isAvailable(name);

	async function getKeyFromClipboard() {
		key = (await readText()) ?? '';
	}

	async function submitKey() {
		loading = true;
		try {
			data = await invokeCommand<ImportData>('import_code', { key: key.trim() });
			name = data.name;
			mode = isAvailable(name) ? 'new' : 'overwrite';
		} finally {
			loading = false;
		}
	}

	async function importData() {
		if (!data) return;

		data.name = name;

		if (mode === 'overwrite') {
			let confirmed = await confirm(`是否确实要覆盖 ${data.name}?`);

			if (!confirmed) return;
		}

		invokeCommand('import_data', { data, importAll }).then(refreshProfiles);
		data = null;
		importAll = false;
		open = false;
	}

	function isAvailable(name: string) {
		return !profiles.some((profile) => profile.name === name);
	}
</script>

<Popup
	title="导入配置文件"
	bind:open
	onClose={() => {
		data = null;
		importAll = false;
	}}
>
	{#if data !== null}
		<TabsMenu
			bind:value={mode}
			options={[
				{ value: 'new', label: '新建' },
				{ value: 'overwrite', label: '覆盖现有' }
			]}
		>
			<Tabs.Content value="new">
				<div class="flex items-center">
					<Label>Profile name</Label>

					<div class="relative grow">
						<InputField bind:value={name} class="w-full" />

						{#if !nameAvailable}
							<Tooltip
								class="absolute right-2 bottom-0 h-full cursor-text text-xl text-red-500"
								side="left"
							>
								<Icon icon="mdi:error" />

								<div slot="tooltip">
									个人资料 {name} 已存在！
								</div>
							</Tooltip>
						{/if}
					</div>

					<Info>导入的配置文件的唯一名称。</Info>
				</div>
			</Tabs.Content>

			<Tabs.Content value="overwrite">
				<div class="flex items-center">
					<Label>Choose profile</Label>

					<Dropdown
						class="grow"
						items={profiles.map((profile) => profile.name)}
						avoidCollisions={false}
						multiple={false}
						bind:selected={name}
					/>

					<Info>要用导入的配置文件覆盖的现有配置文件。</Info>
				</div>
			</Tabs.Content>
		</TabsMenu>

		<details>
			<summary class="mt-2 cursor-pointer text-slate-300"
				>{data.modNames.length}个要安装的 Mod</summary
			>

			<ModCardList names={data.modNames} class="mt-2 max-h-[50vh] shrink grow" />
		</details>

		<details>
			<summary class="mt-1 cursor-pointer text-slate-300">高级选项</summary>

			<div class="mt-1 flex items-center">
				<Label>导入所有文件</Label>
				<Checkbox bind:value={importAll} />
				<Info>
					导入配置文件中找到的所有文件，而不仅仅是众所周知的配置文件格式。
					这是不安全的，可能会让攻击者在您的系统上安装恶意软件。<b >仅对可信配置文件启用此功能！</b >
				</Info>
			</div>
		</details>

		<div class="mt-2 flex w-full items-center justify-end gap-2 text-slate-400">
			<BigButton
				color="slate"
				on:click={() => {
					open = false;
					data = null;
				}}>取消</BigButton
			>
			<BigButton disabled={!nameAvailable || loading} on:click={importData}>导入</BigButton>
		</div>
	{:else}
		<div class="mt-1 flex gap-2">
			<div class="grow">
				<InputField bind:value={key} class="w-full" size="lg" placeholder="输入导入代码..." />
			</div>

			<BigButton on:click={submitKey} disabled={loading}>
				{#if loading}
					<Icon icon="mdi:loading" class="animate-spin" />
				{:else}
					导入
				{/if}
			</BigButton>
		</div>
	{/if}
</Popup>
