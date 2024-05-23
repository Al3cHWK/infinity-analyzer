<template>
  <div :class="['main-container', { 'dark-theme': isDarkTheme }]">
    <div class="content">
      <h1>Анализатор логов</h1>
      <!-- Кнопка переключения темы -->
      <!-- <div class="theme-toggle">
        <img
          :src="isDarkTheme ? '/src/assets/light-mode-icon.png' : '/src/assets/dark-mode-icon.png'"
          class="theme-icon"
          @click="toggleTheme"
        />
      </div> -->
            <!-- Кнопка для выбора файла -->
            <div class="button-container">
              <button @click="selectFile" class="button">Выберите файл</button>
            </div>
      <!-- Контейнер для выбора типа логов с использованием радиокнопок -->
      <div class="container2">
        <div class="tabs-unique">
          <input
            checked
            value="frequent"
            name="log_type"
            id="frequent"
            type="radio"
            class="input-unique"
            v-model="selectedView"
            @change="updateView"
          />
          <label for="frequent" class="label-unique">Самые частые логи</label>
          <input
            value="filtered"
            name="log_type"
            id="filtered"
            type="radio"
            class="input-unique"
            v-model="selectedView"
            @change="updateView"
          />
          <label for="filtered" class="label-unique">Отфильтрованные логи</label>
          <input
            value="frequentErrors"
            name="log_type"
            id="frequentErrors"
            type="radio"
            class="input-unique"
            v-model="selectedView"
            @change="updateView"
          />
          <label for="frequentErrors" class="label-unique">Частые ошибки</label>
        </div>
        <div class="container3" v-if="selectedView === 'filtered'">
          <h3>Фильтр логов</h3>
          <select v-model="selectedFilter" @change="filterLogs">
            <option value="all">Все</option>
            <option value="INFO">Info</option>
            <option value="WARNING">Warning</option>
            <option value="ERROR">Error</option>
            <option value="DEBUG">Debug</option>
          </select>
        </div>
      </div>




            <!-- Поле ввода для регулярного выражения и кнопка возврата логов -->
            <div class="regex-container">
              <input type="text" v-model="regex" placeholder="Введите регулярное выражение" />
              <div class="sub-container">
              <button @click="applyRegexFilter" class="button">Применить</button>
              <button @click="resetLogs" class="button">Сбросить</button>
            </div>
            </div>
      <!-- Блок для отображения результатов анализа -->
      <div v-if="stats">
        <h2>Результат анализа</h2>
        <p>Всего: {{ stats.total }}</p>
        <p>INFO сообщений: {{ stats.info }}</p>
        <p>WARNING сообщений: {{ stats.warning }}</p>
        <p>ERROR сообщений: {{ stats.error }}</p>
        <p>DEBUG сообщений: {{ stats.debug }}</p>
      </div>

      <!-- Кнопки для экспорта данных -->
      <div class="export-buttons">
        <button @click="exportToJson" class="button">Экспорт в JSON</button>
        <button @click="exportToCsv" class="button">Экспорт в CSV</button>
      </div>
    </div>

    <!-- Блок для отображения частых логов -->
    <div v-if="stats && selectedView === 'frequent'" class="log-wrapper">
      <h3 class="text">Самые частые логи</h3>
      <div class="log-container">
        <ul>
          <li v-for="(count, message) in stats.frequent_messages" :key="message" @click="setActiveLog(message + ': ' + count)">
            {{ message }}: {{ count }}
          </li>
        </ul>
      </div>
    </div>

    <!-- Блок для отображения отфильтрованных логов -->
    <div v-if="stats && selectedView === 'filtered'" class="log-wrapper">
      <h3 class="text">Отфильтрованные логи</h3>
      <div class="log-container">
        <ul>
          <li v-for="log in filteredLogs" :key="log" @click="setActiveLog(log)">
            {{ log }}
          </li>
        </ul>
      </div>
    </div>

    <!-- Блок для отображения частых ошибок -->
    <div v-if="stats && selectedView === 'frequentErrors'" class="log-wrapper">
      <h3 class="text">Частые ошибки</h3>
      <div class="log-container">
        <ul>
          <li v-for="error in filteredLogs" :key="error" @click="setActiveLog(error)">
            {{ error }}
          </li>
        </ul>
      </div>
    </div>
  </div>

  <!-- Модальное окно для отображения деталей лога -->
  <div v-if="activeLog" class="modal" @click.self="closeModal">
    <div class="modal-content">
      <span class="close" @click="closeModal">&times;</span>
      <p>{{ activeLog }}</p>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';

export default {
  data() {
    return {
      stats: null, // Объект для хранения статистики логов
      logs: [], // Массив всех логов
      filteredLogs: [], // Массив отфильтрованных логов
      originalLogs: [], // Оригинальные логи для восстановления
      selectedFilter: 'all', // Текущий выбранный фильтр
      selectedView: 'frequent', // Текущий выбранный вид логов
      activeLog: null, // Текущий активный лог для отображения в модальном окне
      isDarkTheme: window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches, // Начальная тема
      regex: '', // Регулярное выражение
    };
  },
  methods: {
    // Метод для выбора файла
    async selectFile() {
      const selectedFile = await open({
        multiple: false,
        filters: [
          { name: 'Log Files', extensions: ['log', 'txt'] }
        ],
      });
      if (selectedFile) {
        this.analyzeFile(selectedFile);
      }
    },
    // Метод для анализа выбранного файла
    async analyzeFile(filePath) {
      const response = await invoke('analyze_logs', { filePath });
      this.stats = response.stats;
      this.logs = response.logs; // Предполагается, что это массив строк
      this.originalLogs = [...this.logs]; // Сохранение оригинальных логов
      this.updateView();
    },
    // Метод для фильтрации логов
    filterLogs() {
      if (this.selectedFilter === 'all') {
        this.filteredLogs = this.logs.map(log => this.formatLogPreview(log));
      } else {
        this.filteredLogs = this.logs.filter(log => log.includes(this.selectedFilter))
                                     .map(log => this.formatLogPreview(log));
      }
    },
    // Метод для обновления вида логов
    updateView() {
      if (this.selectedView === 'filtered') {
        this.filterLogs();
      } else if (this.selectedView === 'frequent') {
        this.filteredLogs = Object.entries(this.stats.frequent_messages)
                                  .sort((a, b) => b[1] - a[1])
                                  .map(([message, count]) => `${message}: ${count}`);
      } else if (this.selectedView === 'frequentErrors') {
        this.filteredLogs = Object.entries(this.stats.frequent_errors)
                                  .sort((a, b) => b[1] - a[1])
                                  .map(([message, count]) => `${message}: ${count}`);
      }
    },
    // Метод для форматирования превью лога
    formatLogPreview(log) {
      return log.length > 100 ? `${log.substring(0, 100)}...` : log;
    },
    // Метод для установки активного лога и отображения его в модальном окне
    setActiveLog(fullLog) {
      this.activeLog = this.logs.find(log => this.formatLogPreview(log) === fullLog) || fullLog;
    },
    // Метод для закрытия модального окна
    closeModal() {
      this.activeLog = null;
    },
    handleKeyUp(event) {
      if (event.key === 'Escape' && this.activeLog) {
        this.closeModal();
      }
    },
    // Метод для переключения темы
    toggleTheme() {
      this.isDarkTheme = !this.isDarkTheme;
      document.documentElement.classList.toggle('dark-theme', this.isDarkTheme);
    },
    // Метод для восстановления полных версий урезанных логов
    getFullLogs(logs) {
      return logs.map(shortLog => {
        const fullLog = this.logs.find(log => log.startsWith(shortLog.substring(0, 100).replace('...', '')));
        return fullLog || shortLog;
      });
    },
    // Метод для экспорта данных в JSON
    exportToJson() {
      const data = this.selectedView === 'filtered' ? this.getFullLogs(this.filteredLogs) : this.logs;
      const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(data));
      const downloadAnchorNode = document.createElement('a');
      downloadAnchorNode.setAttribute("href", dataStr);
      downloadAnchorNode.setAttribute("download", "logs.json");
      document.body.appendChild(downloadAnchorNode); // required for firefox
      downloadAnchorNode.click();
      downloadAnchorNode.remove();
    },
    // Метод для экспорта данных в CSV
    exportToCsv() {
      const data = this.selectedView === 'filtered' ? this.getFullLogs(this.filteredLogs) : this.logs;
      const csvContent = "data:text/csv;charset=utf-8," + data.map(log => log.replace(/,/g, ';')).join("\n");
      const downloadAnchorNode = document.createElement('a');
      downloadAnchorNode.setAttribute("href", csvContent);
      downloadAnchorNode.setAttribute("download", "logs.csv");
      document.body.appendChild(downloadAnchorNode); // required for firefox
      downloadAnchorNode.click();
      downloadAnchorNode.remove();
    },
    // Метод для применения регулярного выражения для фильтрации логов
    applyRegexFilter() {
      if (this.regex) {
        const regex = new RegExp(this.regex, 'i');
        this.logs = this.originalLogs.filter(log => regex.test(log));
        this.updateView();
      }
    },
    // Метод для сброса логов к оригинальным
    resetLogs() {
      this.logs = [...this.originalLogs];
      this.updateView();
    }
  },
  mounted() {
    document.addEventListener('keyup', this.handleKeyUp);
  },
  beforeDestroy() {
    document.removeEventListener('keyup', this.handleKeyUp);
  }
};
</script>

<style scoped>
html, body {
  height: 100%;
  margin: 0;
  font-family: Arial, sans-serif;
  background-color: #f4f4f4;
}

.main-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  text-align: center;
}

.content {
  width: 100%;
  max-width: 600px;
}

.theme-toggle {
  position: absolute;
  top: 10px;
  right: 10px;
}

.theme-icon {
  width: 32px;
  height: 32px;
  cursor: pointer;
}

.button {
  --bg: #000;
  --hover-bg: #0080ff;
  --hover-text: #000;
  color: #fff;
  cursor: pointer;
  border: 1px solid var(--bg);
  border-radius: 4px;
  padding: 0.8em 2em;
  background: var(--bg);
  transition: 0.2s;
  font-weight: bold;
  margin: 10px;
}

.button:hover {
  color: var(--hover-text);
  transform: translate(-0.25rem, -0.25rem);
  background: var(--hover-bg);
  box-shadow: 0.25rem 0.25rem var(--bg);
}

.button:active {
  transform: translate(0);
  box-shadow: none;
}

.regex-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 20px;
  width: 100%;
}

.regex-container input {
  width: 100%;
  max-width: 500px;
  border: none;
  outline: none;
  background: none;
  font-size: 18px;
  color: #555;
  padding: 15px 20px 10px 20px; /* Увеличили правый и левый отступы */
  box-shadow: inset 8px 8px 8px #cbced1, inset -8px -8px 8px #ffffff;
  border-radius: 25px;
  box-sizing: border-box; /* Обеспечивает корректный расчет ширины */
}


.regex-container input::placeholder {
  color: #555;
  transition: all 0.3s ease;
}
.regex-container input:focus::placeholder {
  color: #999;
}

.container3 {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.container2 {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  margin: 10px;
}

.tabs-unique {
  height: 48px;
  display: grid;
  grid-auto-flow: column;
  background: hsl(0, 0%, 100%);
  border-radius: 8px;
  grid-auto-columns: 1fr;
  position: relative;
  border: 4px solid hsl(0, 0%, 100%);
}

.tabs-unique {
  --ease: linear(
    0,
    0.1641 3.52%,
    0.311 7.18%,
    0.4413 10.99%,
    0.5553 14.96%,
    0.6539 19.12%,
    0.738 23.5%,
    0.8086 28.15%,
    0.8662 33.12%,
    0.9078 37.92%,
    0.9405 43.12%,
    0.965 48.84%,
    0.9821 55.28%,
    0.992 61.97%,
    0.9976 70.09%,
    1
  );
}

.tabs-unique > .input-unique,
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}

.tabs-unique:has(:checked:nth-of-type(1)) {
  --active: 0;
}
.tabs-unique:has(:checked:nth-of-type(2)) {
  --active: 1;
}
.tabs-unique:has(:checked:nth-of-type(3)) {
  --active: 2;
}
.tabs-unique:has(:checked:nth-of-type(4)) {
  --active: 3;
}

.tabs-unique :checked + .label-unique {
  --highlight: 1;
  background: hsl(0, 0%, 0%);
  color: hsl(0, 0%, 100%);
}

.tabs-unique:has(.input-unique:nth-of-type(2)) {
  --count: 2;
}
.tabs-unique:has(.input-unique:nth-of-type(3)) {
  --count: 3;
}
.tabs-unique:has(.input-unique:nth-of-type(4)) {
  --count: 4;
}

.tabs-unique .label-unique {
  padding: 0 clamp(10px, 10px + 10px, 20px);
  cursor: pointer;
  text-align: center;
  height: 100%;
  display: grid;
  border-radius: calc(8px - 4px);
  place-items: center;
  color: hsl(0, 0%, 0% / calc(0.5 + var(--highlight, 0)));
  transition: background, color;
  transition-duration: 0.25s;
  transition-timing-function: var(--ease, ease);
}

.input-unique:not(:checked) + .label-unique {
  background: hsl(0, 0%, 100%);
  color: hsl(0, 0%, 0%);
}

.input-unique:not(:checked) + .label-unique:hover {
  --highlight: 0.35;
  background: hsl(0, 0%, 80%);
}

.tabs-unique::after {
  pointer-events: none;
  content: "";
  width: calc(100% / var(--count));
  height: 100%;
  background: hsl(0, 0%, 0%);
  position: absolute;
  border-radius: calc(8px - 4px);
  mix-blend-mode: difference;
  translate: calc(var(--active, 0) * 100%) 0;
  transition: translate, outline-color;
  transition-duration: 0.25s;
  transition-timing-function: var(--ease, ease);
  outline: 2px solid transparent;
}

.tabs-unique:has(:focus-visible)::after {
  outline-color: red;
}

/* Светлая тема по умолчанию */
.log-wrapper {
  background-color: #fff;
  color: #000;
  border: 2px solid #007bff;
  border-radius: 5px;
  padding: 10px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  width: 100%;
  max-width: 80%;
  margin: 20px 0;
}

.log-container {
  background-color: #f4f4f4;
  color: #000;
  max-height: 300px;
  overflow-y: auto;
  width: 100%;
}

.log-container ul {
  list-style: none;
  padding: 0;
  margin: 0;
  text-align: left;
}

.log-container li {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  padding: 10px;
  border-bottom: 1px solid #ddd;
  cursor: pointer;
}

.log-container li:hover {
  background-color: #eaeaea;
}

.modal {
  background-color: rgba(255, 255, 255, 0.8);
  display: block;
  position: fixed;
  z-index: 1000;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  overflow: auto;
}

.modal-content {
  background-color: #fff;
  color: #000;
  border: 1px solid #ccc;
  margin: 10% auto;
  padding: 20px;
  width: 50%;
}

.close {
  color: #000;
  float: right;
  font-size: 28px;
  font-weight: bold;
}

.close:hover,
.close:focus {
  color: #555;
  text-decoration: none;
  cursor: pointer;
}

/* Темная тема */
@media (prefers-color-scheme: dark) {
  .log-wrapper {
    background-color: #333;
    color: #fff;
    border: 2px solid #007bff;
  }

  .log-container {
    background-color: #444;
    color: #fff;
  }

  .log-container li {
    border-bottom: 1px solid #555;
  }

  .log-container li:hover {
    background-color: #555;
  }

  .modal {
    background-color: rgba(0, 0, 0, 0.8);
  }

  .modal-content {
    background-color: #222;
    color: #fff;
    border: 1px solid #444;
  }

  .close {
    color: #aaa;
  }

  .close:hover,
  .close:focus {
    color: #fff;
  }
}

.dark-theme .log-wrapper {
  background-color: #333;
  color: #fff;
  border: 2px solid #007bff;
}

.dark-theme .log-container {
  background-color: #444;
  color: #fff;
}

.dark-theme .log-container li {
  border-bottom: 1px solid #555;
}

.dark-theme .log-container li:hover {
  background-color: #555;
}

.dark-theme .modal {
  background-color: rgba(0, 0, 0, 0.8);
}

.dark-theme .modal-content {
  background-color: #222;
  color: #fff;
  border: 1px solid #444;
}

.dark-theme .close {
  color: #aaa;
}

.dark-theme .close:hover,
.dark-theme .close:focus {
  color: #fff;
}
</style>
