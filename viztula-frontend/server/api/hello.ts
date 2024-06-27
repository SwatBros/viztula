export default defineEventHandler((event) => {
  return fetch('http://localhost:8999/chart/data', {
    method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({
      db: 'dvdrental',
      table: 'actor',
      columns: ["first_name", "last_name"]
    })
  })
})