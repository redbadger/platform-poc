# Inventory service

Query specific sku codes to check if they're in stock

Send a GET request to `/inventory` specifying a comma separated list of skus with `skus` query param.

Sample request:

```shell
curl "localhost:8080/inventory?skus=WND-WPR-AW,RAD-HI-EFF,BAT-HC-12V,FL-PMP-ELEC,HDL-LED-LONG,SUS-KIT-IMP,AIR-FLT-PREM,TIR-SET-AS,SPK-PLG-HI-EFF,EXH-SYS-PERF,BRK-PD-HP,TRB-CHR-HP,ENG-V8-500,ALT-HO-ENH"
```

This is a full list of skus initialised by the data-init component.
