export default defineEventHandler((event): Promise<any> => {
  return $fetch('http://localhost:8999/chart/data', {
    method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({
      db: 'dvdrental',
      table: 'payment',
      columns: ["payment_date", "SUM(amount)"],
      group_by: ["payment_date"],
      order_by: ["payment_date"],
      limit: 10
    })
  })
})