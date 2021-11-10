<script lang="ts">
    import {createEventDispatcher} from "svelte";
    import Poll from '../models/Poll'

    const dispatch = createEventDispatcher();
    export let poll: Poll;
    const pollSelected = () => {
        dispatch('click', poll)
    }
</script>

<article class="mdc-card poll-card" on:click={pollSelected}>
    <div class="mdc-card__primary-action mdc-card__content poll-card-content">
        <h3>{poll.title}</h3>
        <mwc-list noninteractive>
            {#each poll.options as option}
                <mwc-list-item>{option.text}</mwc-list-item>
            {/each}
        </mwc-list>
        <div class="mdc-card__ripple"></div>
    </div>
</article>

<style>
    .poll-card {
        width: 24rem;
    }

    .poll-card-content {
        padding: 1rem;
    }

    .poll-card-content h3 {
        padding: 1rem 0;
    }
</style>
