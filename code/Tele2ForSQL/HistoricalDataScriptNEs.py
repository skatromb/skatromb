from datetime import date
from calendar import Calendar


ENV = 'DEV'  # 'PRD2'
if ENV == 'DEV':
    Layer = 'DDS'
    Postfix_prc = '_LIVADNYY'
    Postfix_tbl = '_2'
else:
    Layer = 'MDS'
    Postfix_prc = ''
    Postfix_tbl = ''


def generate_sql_for_month(year: date.year, month: date.month):

    def call_procedure_without_params(procedure_name: str, env: str = ENV, layer: str = Layer, postfix: str = Postfix_prc) -> str:
        call_statement = 'CALL ' + env + '_' + layer + '.' + procedure_name + postfix
        return call_statement

    print('\n-- ' + str(year) + '-' + str(month) + ' ' + '-' * 64 + '\n')

    # Блок генерации кода на каждый день месяца
    dates = Calendar()
    for day in [i for i in dates.itermonthdates(year, month) if i.month == month]:
        print(call_procedure_without_params('LOAD_NE_SUBS_REVENUE_DATE', layer='DDS') +
              "(7273889, date'" + day.isoformat() + "');")

    print('COLLECT STATS ON ' + ENV + '_' + 'DDS.NE_SUBS_REVENUE_DATE' + Postfix_tbl + ';')

    # Блок месячных расчётов
    print()
    params = "(date'" + date(year, month, 1).isoformat() + "', 7273889);"

    print(call_procedure_without_params('LOAD_NETWORK_ELEMENT_MONTHLY_METRICS') + params)
    print(call_procedure_without_params('LOAD_SITE_MONTHLY_METRICS') + params)

    # Удаляем информацию за рассчитанный месяц
    print()
    date_str = "'" + date(year, month, 1).isoformat() + "'"
    print("DELETE " + ENV + "_DDS.NE_SUBS_REVENUE_DATE" + Postfix_tbl + " WHERE REPORT_DATE BETWEEN "
          "date" + date_str + " AND ADD_MONTHS(date" + date_str + ", 1) - 1;")


# 2019 с октября
for Month in range(10, 0, -1):
    generate_sql_for_month(year=2019, month=Month)

# 2018 до февраля
for Month in range(12, 1, -1):
    generate_sql_for_month(year=2018, month=Month)