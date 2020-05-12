<template>
  <v-container>
    <v-row class="text-center">
      <v-col cols="12">
        <v-simple-table>
          <template v-slot:default>
            <thead>
              <tr>
                <th class="text-left">Name</th>
                <th class="text-left">Size</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="item in files" :key="item.name">
                <td>{{ item.name }}</td>
                <td>{{ item.size }}</td>
              </tr>
            </tbody>
          </template>
        </v-simple-table>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Home extends Vue {
  private files: Array<Record<string, number>> = [];

  mounted() {
    for (let i = 0; i <= 20; i++) {
      this.files.push({ name: i, size: i});
    }
  }
}
</script>
