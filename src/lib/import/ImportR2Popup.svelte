<script lang="ts">
	import BigButton from '$lib/components/BigButton.svelte';
	import Popup from '$lib/components/Popup.svelte';
	import type { R2ImportData } from '$lib/models';
	import ImportR2Flow from './ImportR2Flow.svelte';

	export let open: boolean;

	let loading = false;
	let importFlow: ImportR2Flow;
	let importData: R2ImportData;

	$: if (open && importFlow) {
		importFlow.refresh(null);
	}

	async function doImport() {
		if (await importFlow.doImport()) {
			open = false;
		}
	}
</script>

<Popup bind:open title="Import profiles from other manager" canClose={!loading}>
	<div class="mb-2 text-slate-300">
		<p>
			这将从 r2modman 或 Thunderstore Mod Manager 导入<b>当前游戏的</b>配置文件。
		</p>

		<p class="mt-2">
			<b>请勿在导入过程中关闭 Gale。</b>
		</p>
	</div>
	<ImportR2Flow bind:this={importFlow} bind:loading bind:importData />

	<div class="mt-3 mr-0.5 flex w-full justify-end gap-2">
		<BigButton color="slate" on:click={() => (open = false)}>取消</BigButton>
		<BigButton color="accent" on:click={doImport}>导入</BigButton>
	</div>
</Popup>
