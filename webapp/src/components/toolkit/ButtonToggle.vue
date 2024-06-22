<template>
  <q-btn-toggle
    class="prodo-toggle"
    v-model="selected"
    @click="onClick"
    no-caps
    spread
    rounded
    unelevated
    :options="props.options"
  />
</template>

<script lang="ts" setup>
import { ref, defineProps, defineEmits } from 'vue'

const props = defineProps({
  options: {
    // type: Array as PropType<{ label?: string; value: any }[]>,
    required: true
  },
  modelValue: {
    required: true
  }
})

const selected = ref(props.modelValue)

const emit = defineEmits(['update:modelValue'])

const onClick = () => {
  emit('update:modelValue', selected)
}
</script>

<style lang="scss">
.prodo-toggle {
  border: 1px solid $border;

  button {
    border-radius: 17px !important;
    margin: 2px;
  }

  button.bg-primary {
    color: black !important;
    border: 2px solid $border;
    background: transparent !important;
  }
}

.body--dark {
  .prodo-toggle {
    border: 1px solid $border-dark;

    button.bg-primary {
      color: white !important;
      border: 2px solid $border-dark;
    }
  }
}

.q-btn-group--rounded {
  border-radius: 20px;

  .q-btn-item {
    padding: 4px;
  }
}
</style>
