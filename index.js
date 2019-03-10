const Page = {
  data() {
    return ({
      title: "",
      problem_link: "",
      oj: "",
      scrapbox_link: "",
      data: "",
    });
  },
  props: {
    page: Object,
  },
  created() {
    this.title = this.page["title"];
    this.scrapbox_link = `https://scrapbox.io/ia7ck-comp-pro/${this.title}`;
    this.oj = get_oj(this.page["descriptions"]);
    this.problem_link = this.get_problem_link(this.page["descriptions"]);
    this.date = dateFns.format(new Date(this.page["created"] * 1000), "YYYY-MM-DD HH:mm");
  },
  methods: {
    get_problem_link(descriptions) {
      let link = "";
      for (let i = 0; i < descriptions.length; i++) {
        if (descriptions[i].startsWith("http")) {
          link = descriptions[i];
          break;
        }
      }
      return link;
    },
  },
  template: `
    <li>
      <span>
        <a v-if="problem_link.length > 0" :href="problem_link">{{ title }}</a>
        <span v-else>{{ title }}</span>
      </span>
      <span>[{{ oj }}]</span>
      <span><a :href="scrapbox_link">解説</a></span>
      <span>{{ this.date }}</span>
    </li>
  `
};

const app = new Vue({
  el: "#app",
  components: {
    "page": Page,
  },
  data: {
    pages: [],
  },
  async created() {
    const resp = await fetch("https://script.google.com/macros/s/AKfycbzSFMM_St_VhociIILfWgjYE4Yv7ZBsx2jFQdkwYXweza0X6Uk/exec?url=https://scrapbox.io/api/pages/ia7ck-comp-pro?limit=300"); // limitは適当に増やす
    const json = await resp.json();
    this.pushPages(json["pages"]);
  },
  methods: {
    pushPages(pages) {
      pages.forEach((page) => {
        this.pages.push({
          id: page["id"],
          title: page["title"],
          descriptions: page["descriptions"],
          created: page["created"],
        });
      }, this);
      this.pages.sort((a, b) => (b["created"] - a["created"]));
    },
  },
  computed: {
    validPages() {
      return this.pages.filter((page) => {
        let oj = get_oj(page["descriptions"]).toLowerCase();
        let ojs = ["atcoder", "codeforces", "csacademy", "yukicoder", "aoj", "leetcode"];
        return ojs.includes(oj);
      });
    },
  },
  template: `
    <div class="container" style="padding: 1.5rem;">
        <ul>
          <page v-for="page in validPages" :key="page.id" :page="page">
          </page>
        </ul>
    </div>
  `
});

function get_oj(descriptions) {
  if (descriptions.length === 0) {
    return "";
  }
  let oj = descriptions[0].split(" ")[0];
  if (!oj.startsWith("#")) {
    return "";
  }
  return oj.slice(1);
}
