<script lang="ts">
	import remarkGfm from 'remark-gfm';
	import type { Plugin } from 'svelte-exmarkdown';
	import Markdown from 'svelte-exmarkdown';
	import typescript from 'highlight.js/lib/languages/typescript';
	import csharp from 'highlight.js/lib/languages/csharp';
	import bash from 'highlight.js/lib/languages/bash';
	import rust from 'highlight.js/lib/languages/rust';
	import sql from 'highlight.js/lib/languages/sql';
	import rehypeHighlight from 'rehype-highlight';
	import type { Note } from '$lib/types';
	import { mode } from 'mode-watcher';
	import { viewMode } from '$lib/stores';

	const plugins: Plugin[] = [
		{
			rehypePlugin: [
				rehypeHighlight,
				{ ignoreMissing: true, languages: { typescript, csharp, rust, bash, sql } }
			],
			remarkPlugin: [remarkGfm]
		}
	];

	export let note: Note;
	export let save: (note: Note) => Promise<void>;

	function handleTab(event: KeyboardEvent) {
		if (event.key === 'Tab') {
			event.preventDefault();

			const textarea = event.target as HTMLTextAreaElement;
			const start = textarea.selectionStart;
			const end = textarea.selectionEnd;

			// Insert a tab character at the cursor position
			note.content = note.content.substring(0, start) + '    ' + note.content.substring(end);

			// Move the cursor after the inserted tab character
			setTimeout(() => {
				textarea.selectionStart = textarea.selectionEnd = start + 4;
			}, 0);
		}
	}
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="container">
	{#if $viewMode !== 'preview'}
		<textarea
			bind:value={note.content}
			class="editor"
			autocomplete="off"
			autocorrect="off"
			autocapitalize="off"
			spellcheck="false"
			on:keydown={handleTab}
			on:input={() => save(note)}
		></textarea>
	{/if}
	{#if $viewMode !== 'editor'}
		<div class="markdown-body">
			<Markdown bind:md={note.content} {plugins} />
		</div>
	{/if}
</div>

<svelte:head>
	{#if $mode === 'dark'}
		<link
			rel="stylesheet"
			href="//cdn.jsdelivr.net/npm/@catppuccin/highlightjs@1.0.0/css/catppuccin-macchiato.css"
		/>
	{:else}
		<link
			rel="stylesheet"
			href="//cdn.jsdelivr.net/npm/@catppuccin/highlightjs@1.0.0/css/catppuccin-latte.css"
		/>
	{/if}
</svelte:head>

<style>
	.container {
		width: 100%;
		display: flex;
		flex-direction: row;
		height: 100%;
		padding-bottom: 0;
		max-height: calc(100vh - 80px);
	}

	.markdown-body {
		flex: 1;
		overflow: auto;
		padding: 10px;
	}

	textarea.editor {
		width: 100%;
		flex: 1;
		height: 100%;
		resize: none;
		padding: 10px;
		border: none;
		border-right: 1px solid;
		outline: none;
	}
</style>
