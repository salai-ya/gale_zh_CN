<script lang="ts">
	import { onMount } from 'svelte';
	import Icon from '@iconify/svelte';
	import { Button, Menubar } from 'bits-ui';

	import MenubarItem from '$lib/menu/MenubarItem.svelte';

	import InputField from '$lib/components/InputField.svelte';
	import BigButton from '$lib/components/BigButton.svelte';
	import Popup from '$lib/components/Popup.svelte';

	import ImportR2Popup from '$lib/import/ImportR2Popup.svelte';
	import ExportCodePopup from '$lib/import/ExportCodePopup.svelte';
	import ImportProfilePopup from '$lib/import/ImportProfilePopup.svelte';

	import AboutPopup from './AboutPopup.svelte';
	import MenubarMenu from './MenubarMenu.svelte';
	import NewProfilePopup from './NewProfilePopup.svelte';
	import MenubarSeparator from './MenubarSeparator.svelte';

	import { capitalize } from '$lib/util';
	import { invokeCommand } from '$lib/invoke';
	import type { ImportData } from '$lib/models';
	import { activeProfile, refreshProfiles } from '$lib/stores';

	import { confirm, open } from '@tauri-apps/plugin-dialog';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { open as shellOpen } from '@tauri-apps/plugin-shell';
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';

	let importR2Open = false;
	let newProfileOpen = false;
	let exportCodePopup: ExportCodePopup;

	let importProfileOpen = false;
	let importProfileData: ImportData | null = null;

	let profileOperation: 'rename' | 'duplicate' = 'rename';
	let profileOperationName = '';
	let profileOperationOpen = false;
	let profileOperationInProgress = false;

	let aboutOpen = false;

	const appWindow = getCurrentWindow();

	async function importLocalMod() {
		let path = await open({
			title: '选择要导入的 mod 文件',
			filters: [{ name: 'Dll or zip', extensions: ['dll', 'zip'] }]
		});

		if (path === null) return;
		await invokeCommand('import_local_mod', { path });
		await refreshProfiles();
	}

	async function importFile() {
		let path = await open({
			title: '选择要导入的文件',
			filters: [{ name: 'Profile file', extensions: ['r2z'] }]
		});

		if (path === null) return;
		let data = await invokeCommand<ImportData>('import_file', { path });

		importProfileData = data;
		importProfileOpen = true;
	}

	async function exportFile() {
		let dir = await open({
			directory: true,
			title: '选择要将配置文件导出到的文件夹'
		});

		if (dir === null) return;
		invokeCommand('export_file', { dir });
	}

	async function setAllModsState(enable: boolean) {
		await invokeCommand('set_all_mods_state', { enable });
		activeProfile.update((profile) => profile);
	}

	function openProfileOperation(operation: 'rename' | 'duplicate') {
		profileOperation = operation;
		profileOperationName = $activeProfile?.name ?? 'Unknown';
		profileOperationOpen = true;
	}

	async function doProfileOperation() {
		if (profileOperationInProgress) return;

		profileOperationInProgress = true;

		try {
			if (profileOperation == 'rename') {
				await invokeCommand('rename_profile', { name: profileOperationName });
			} else if (profileOperation == 'duplicate') {
				await invokeCommand('duplicate_profile', { name: profileOperationName });
			}
		} catch (e) {
			profileOperationInProgress = false;
			throw e;
		}

		await refreshProfiles();
		profileOperationInProgress = false;
		profileOperationOpen = false;
	}

	async function uninstallDisabledMods() {
		let confirmed = await confirm('您确定要卸载所有已禁用的模组吗？');
		if (!confirmed) return;

		await invokeCommand<number>('remove_disabled_mods');
		await refreshProfiles();
	}

	async function zoom(value: { delta: number } | { factor: number }) {
		await invokeCommand('zoom_window', { value });
	}

	async function copyLaunchArgs() {
		let str = await invokeCommand<string>('get_launch_args');
		writeText(str);
	}

	async function clearModCache() {
		let result = await confirm(
			"您确定要删除所有缓存的 Mod 吗？这可能会使已安装的 Mod 使用的磁盘空间增加一倍。只有在您知道自己在做什么的情况下才继续！"
		);

		if (!result) return;

		await invokeCommand('clear_download_cache', { soft: false });
	}

	const hotkeys: { [key: string]: () => void } = {
		'+': () => zoom({ delta: 0.25 }),
		'-': () => zoom({ delta: -0.25 }),
		'0': () => zoom({ factor: 1 }),
		n: () => (newProfileOpen = true),
		d: () => openProfileOperation('duplicate')
	};

	onMount(() => {
		document.onkeydown = ({ key, ctrlKey }) => {
			if (key === 'F2') {
				openProfileOperation('rename');
				return;
			}

			if (!ctrlKey) return;

			const hotkey = hotkeys[key];
			if (hotkey !== undefined) hotkey();
		};
	});
</script>

<header data-tauri-drag-region class="flex h-8 shrink-0 bg-slate-800">
	<Menubar.Root class="flex items-center py-1">
		<img src="favicon.png" alt="Gale logo" class="mr-2 ml-4 h-5 w-5 opacity-50" />
		<MenubarMenu label="文件">
			<MenubarItem on:click={() => invokeCommand('open_profile_dir')} text="打开配置文件文件夹" />
			<MenubarItem on:click={() => invokeCommand('open_game_dir')} text="打开游戏文件夹" />
			<MenubarSeparator />
			<MenubarItem on:click={() => invokeCommand('open_game_log')} text="打开游戏日志" />
			<MenubarItem on:click={() => invokeCommand('open_gale_log')} text="打开 Gale 日志" />
			<MenubarSeparator />
			<MenubarItem on:click={clearModCache} text="清除 Mod 缓存" />
			<MenubarItem
				on:click={() => invokeCommand('clear_download_cache', { soft: true })}
				text="清除未使用的 Mod 缓存"
			/>
			<MenubarItem on:click={() => invokeCommand('trigger_mod_fetch')} text="获取 Mod" />
		</MenubarMenu>
		<MenubarMenu label="配置文件">
			<MenubarItem
				on:click={() => (newProfileOpen = true)}
				text="创建新配置文件"
				key="Ctrl N"
			/>
			<MenubarItem
				on:click={() => openProfileOperation('rename')}
				text="重命名活动配置文件"
				key="F2"
			/>
			<MenubarItem
				on:click={() => openProfileOperation('duplicate')}
				text="复制活动配置文件"
				key="Ctrl D"
			/>
			<MenubarSeparator />
			<MenubarItem on:click={() => invokeCommand('copy_dependency_strings')} text="复制 mod 列表" />
			<MenubarItem on:click={() => invokeCommand('copy_debug_info')} text="复制调试信息" />
			<MenubarItem on:click={copyLaunchArgs} text="复制启动参数" />
			<MenubarSeparator />
			<MenubarItem on:click={() => setAllModsState(true)} text="启用所有 Mod" />
			<MenubarItem on:click={() => setAllModsState(false)} text="禁用所有 Mod" />
			<MenubarItem on:click={uninstallDisabledMods} text="卸载已禁用的 Mod" />
		</MenubarMenu>
		<MenubarMenu label="导入">
			<MenubarItem on:click={() => (importProfileOpen = true)} text="...从代码创建 profile" />
			<MenubarItem on:click={importFile} text="...从文件创建 profile" />
			<MenubarItem on:click={importLocalMod} text="...导入本地模组" />
			<MenubarItem on:click={() => (importR2Open = true)} text="...导入 R2Modman 的配置文件" />
		</MenubarMenu>
		<MenubarMenu label="导出">
			<MenubarItem on:click={() => exportCodePopup.open()} text="...导出配置到代码" />
			<MenubarItem on:click={exportFile} text="...导出配置到文件" />
		</MenubarMenu>
		<MenubarMenu label="窗口">
			<MenubarItem
				on:click={() => invokeCommand('zoom_window', { value: { delta: 0.25 } })}
				text="放大"
				key="Ctrl +"
			/>
			<MenubarItem
				on:click={() => invokeCommand('zoom_window', { value: { delta: -0.25 } })}
				text="缩小"
				key="Ctrl -"
			/>
			<MenubarItem
				on:click={() => invokeCommand('zoom_window', { value: { factor: 1 } })}
				text="重置缩放"
				key="Ctrl 0"
			/>
		</MenubarMenu>
		<MenubarMenu label="Help">
			<MenubarItem
				on:click={() => shellOpen('https://github.com/Kesomannen/ModManager/issues/')}
				text="报告 bug"
			/>
			<MenubarItem
				on:click={() => shellOpen('https://discord.gg/sfuWXRfeTt')}
				text="加入 discord 服务器"
			/>
			<MenubarItem on:click={() => (aboutOpen = true)} text="关于 Gale" />
		</MenubarMenu>
	</Menubar.Root>

	<Button.Root class="group ml-auto px-3 py-1.5 hover:bg-slate-700" on:click={appWindow.minimize}>
		<Icon icon="mdi:minimize" class="text-slate-500 group-hover:text-white" />
	</Button.Root>
	<Button.Root class="group px-3 py-1.5 hover:bg-slate-700" on:click={appWindow.toggleMaximize}>
		<Icon icon="mdi:maximize" class="text-slate-500 group-hover:text-white" />
	</Button.Root>
	<Button.Root class="group px-3 py-1.5 hover:bg-red-700" on:click={appWindow.close}>
		<Icon icon="mdi:close" class="text-slate-500 group-hover:text-white" />
	</Button.Root>
</header>

<Popup
	title="{capitalize(profileOperation)} profile"
	canClose={!profileOperationInProgress}
	bind:open={profileOperationOpen}
>
	<p class="mb-1 text-slate-300">
		{profileOperation == 'duplicate'
			? '输入复制的配置文件的名称:'
			: '输入配置文件的新名称:'}
	</p>
	<InputField
		bind:value={profileOperationName}
		placeholder="输入名称..."
		size="lg"
		class="w-full"
		on:submit={doProfileOperation}
	/>
	<div class="mt-2 ml-auto flex justify-end gap-2">
		{#if !profileOperationInProgress}
			<BigButton color="slate" on:click={() => (profileOperationOpen = false)}>Cancel</BigButton>
		{/if}
		<BigButton
			color="accent"
			fontWeight="medium"
			disabled={profileOperationInProgress}
			on:click={doProfileOperation}
		>
			{#if profileOperationInProgress}
				<Icon icon="mdi:loading" class="my-1 animate-spin text-lg" />
			{:else}
				{capitalize(profileOperation)}
			{/if}
		</BigButton>
	</div>
</Popup>

<AboutPopup bind:open={aboutOpen} />
<ImportR2Popup bind:open={importR2Open} />
<NewProfilePopup bind:open={newProfileOpen} />
<ExportCodePopup bind:this={exportCodePopup} />
<ImportProfilePopup bind:open={importProfileOpen} bind:data={importProfileData} />
