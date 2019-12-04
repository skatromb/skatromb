from datetime import date
from calendar import Calendar


Env = 'DEV'
if Env == 'DEV':
    Layer = 'DDS'
    Postfix = '_LIVADNYY'
else:
    Layer = 'MDS'
    Postfix = ''


def generate_sql_for_month(year: date.year, month: date.month):

    def call_procedure_without_params(procedure_name: str, env: str = Env, layer: str = Layer, postfix: str = Postfix) -> str:
        call_statement = 'CALL ' + env + '_' + layer + '.' + procedure_name + postfix
        return call_statement

    print('-' * 120)

    # Блок генерации кода на каждый день месяца
    dates = Calendar()
    for day in [i for i in dates.itermonthdates(year, month) if i.month == month]:
        print(call_procedure_without_params('LOAD_NE_SUBS_REVENUE') +
              '(7273889, date\'' + day.isoformat() + '\');')

    # Блок месячных расчётов
    print()
    params = '(7273889, date\'' + date(year, month, 1).isoformat() + '\');'

    print(call_procedure_without_params('LOAD_NETWORK_ELEMENT_MONTHLY_METRICS') + params)
    print()
    print(call_procedure_without_params('LOAD_SITE_MONTHLY_METRICS') + params)

    print('-' * 120)


generate_sql_for_month(year=2019, month=9)
