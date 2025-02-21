<script lang="ts">
	import GameSelection from '$lib/menu/GameSelection.svelte';
	import Popup from '$lib/components/Popup.svelte';
	import BigButton from '$lib/components/BigButton.svelte';
	import PathPref from '$lib/prefs/PathPref.svelte';

	import type { Prefs, R2ImportData } from '$lib/models';

	import { invokeCommand } from '$lib/invoke';
	import { onMount } from 'svelte';
	import ImportR2Flow from '$lib/import/ImportR2Flow.svelte';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import AccentColorPref from '$lib/prefs/AccentColorPref.svelte';

	export let open = false;

	let stage: 'gameSelect' | 'importProfiles' | 'settings' | 'end' = 'gameSelect';

	let importFlow: ImportR2Flow;
	let importData: R2ImportData | null | undefined;

	let prefs: Prefs | null = null;

	onMount(async () => {
		if (await invokeCommand<boolean>('is_first_run')) {
			open = true;
			prefs = await invokeCommand('get_prefs');
		}
	});

	async function onSelectGame() {
		try {
			importData = await invoke('get_r2modman_info');
		} catch {
			importData = null;
		}

		stage = importData === null ? 'settings' : 'importProfiles';
	}

	async function importProfiles() {
		if (await importFlow.doImport()) {
			stage = 'settings';
		}
	}

	function set<T>(update: (value: T, prefs: Prefs) => void) {
		return async (value: T) => {
			if (prefs === null) return;

			update(value, prefs);
			await invokeCommand('set_prefs', { value: prefs });
		};
	}
</script>

<Popup title="Welcome to Gale!" canClose={stage === 'end'} bind:open>
	<div class="text-slate-300">
		{#if stage === 'gameSelect'}
			To get started, select a game to mod:
			<GameSelection onSelect={onSelectGame} />
		{:else if stage === 'importProfiles'}
			<p>You can automatically transfer profiles from another mod manager to Gale.</p>

			<p class="mt-1">
				您始终可以稍后通过转到 <b>导入 &gt;...来自 R2Modman</b>.
			</p>

			<ImportR2Flow bind:importData bind:this={importFlow} />

			<div class="mt-2 flex gap-1.5">
				<BigButton color="slate" class="mr-auto" on:click={() => (stage = 'gameSelect')}
					>返回</BigButton
				>
				<BigButton color="slate" on:click={() => (stage = 'settings')}>跳过</BigButton>
				<BigButton color="accent" on:click={importProfiles}>导入</BigButton>
			</div>
		{:else if stage === 'settings'}
			<p>
				让我们确保您的设置符合您的喜好。
				<br />
				您以后可以随时到 <Icon icon="mdi:settings" class="mb-1 inline" />
				<b>设置</b>.
			</p>

			<div class="mt-3 flex flex-col gap-1">
				{#if prefs !== null}
					<PathPref
						label="Gale 数据文件夹"
						type="dir"
						value={prefs.dataDir}
						set={set((value, prefs) => (prefs.dataDir = value))}
					>
						存储 mod 和 profiles 的文件夹。
					</PathPref>

					<PathPref
						label="Steam 库位置"
						type="dir"
						value={prefs.steamLibraryDir}
						set={set((value, prefs) => (prefs.steamLibraryDir = value))}
					>
						默认 Steam 游戏库的路径。
					</PathPref>
				{/if}
			</div>

			<div class="mt-3 flex justify-between">
				<BigButton
					color="slate"
					on:click={() => (stage = importData === null ? 'gameSelect' : 'importProfiles')}
					>返回</BigButton
				>
				<BigButton color="accent" on:click={() => (stage = 'end')}>下一个</BigButton>
			</div>
		{:else if stage === 'end'}
			<p>就是这样，你已经准备好开始修改了！</p>

			<p class="mt-1">
				如果您有任何问题或需要帮助，请随时在 <a
					href="https://discord.gg/sfuWXRfeTt"
					target="_blank"
					class="text-accent-400 hover:underline">Discord 服务器</a
				>联系我们。
			</p>
		{/if}
	</div>
</Popup>
