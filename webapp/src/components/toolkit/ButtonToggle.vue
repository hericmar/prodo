<template>
  <q-btn-toggle
    class="prodo-toggle"
    v-model="selected"
    @click="onClick"
    no-caps
    spread
    rounded
    unelevated
    text-color="primary"
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
  border: 1px solid $primary
}

.q-btn-group--rounded {
  border-radius: 20px;

  .q-btn-item {
    padding: 4px;
  }

  /*
  .q-btn__content {
    background: #9C27B0;
    border-radius: 16px;
  }
   */
}
</style>
