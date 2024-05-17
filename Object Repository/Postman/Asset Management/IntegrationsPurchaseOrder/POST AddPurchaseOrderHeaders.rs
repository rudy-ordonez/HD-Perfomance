<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST AddPurchaseOrderHeaders</name>
   <tag></tag>
   <elementGuidId>2cdb4bdf-7c64-40b4-84c6-613861a11872</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[ { \&quot;OrderNumber\&quot;: \&quot;16662660B\&quot;,\r\n    \&quot;Status\&quot;: \&quot;Open\&quot;,\r\n    \&quot;VendorID\&quot;: \&quot;44646\&quot;,\r\n    \&quot;VendorName\&quot;: \&quot;DELL MARKETING L.P.\&quot;,\r\n    \&quot;SiteID\&quot;: \&quot;District\&quot;,\r\n    \&quot;PurchaseDate\&quot;: \&quot;2018-08-18T00:00:00.000Z\&quot;,\r\n    \&quot;EstimatedDeliveryDate\&quot;: null,\r\n    \&quot;Notes\&quot;: \&quot;Hayes Integration 2018-8-28 \&quot;,\r\n    \&quot;Other1\&quot;: null,\r\n    \&quot;StateFunding\&quot;: \&quot;99.99\&quot;,\r\n    \&quot;FederalFunding\&quot;: \&quot;0\&quot; },\r\n  { \&quot;OrderNumber\&quot;: \&quot;16662661B\&quot;,\r\n    \&quot;Status\&quot;: \&quot;Open\&quot;,\r\n    \&quot;VendorID\&quot;: \&quot;44646\&quot;,\r\n    \&quot;VendorName\&quot;: \&quot;DELL MARKETING L.P.\&quot;,\r\n    \&quot;SiteID\&quot;: \&quot;District\&quot;,\r\n    \&quot;PurchaseDate\&quot;: \&quot;2018-08-18T00:00:00.000Z\&quot;,\r\n    \&quot;EstimatedDeliveryDate\&quot;: null,\r\n    \&quot;Notes\&quot;: \&quot;Hayes Integration 2018-8-28 \&quot;,\r\n    \&quot;Other1\&quot;: null,\r\n    \&quot;StateFunding\&quot;: \&quot;50.5\&quot;,\r\n    \&quot;FederalFunding\&quot;: \&quot;49\&quot; },\r\n  { \&quot;OrderNumber\&quot;: \&quot;16662662B\&quot;,\r\n    \&quot;Status\&quot;: \&quot;Open\&quot;,\r\n    \&quot;VendorID\&quot;: \&quot;44646\&quot;,\r\n    \&quot;VendorName\&quot;: \&quot;DELL MARKETING L.P.\&quot;,\r\n    \&quot;SiteID\&quot;: \&quot;District\&quot;,\r\n    \&quot;PurchaseDate\&quot;: \&quot;2018-08-18T00:00:00.000Z\&quot;,\r\n    \&quot;EstimatedDeliveryDate\&quot;: null,\r\n    \&quot;Notes\&quot;: \&quot;Hayes Integration 2018-8-28 \&quot;,\r\n    \&quot;Other1\&quot;: null,\r\n    \&quot;StateFunding\&quot;: \&quot;10.25\&quot;,\r\n    \&quot;FederalFunding\&quot;: \&quot;56.25\&quot; },\r\n  { \&quot;OrderNumber\&quot;: \&quot;16662663B\&quot;,\r\n    \&quot;Status\&quot;: \&quot;Open\&quot;,\r\n    \&quot;VendorID\&quot;: \&quot;44646\&quot;,\r\n    \&quot;VendorName\&quot;: \&quot;DELL MARKETING L.P.\&quot;,\r\n    \&quot;SiteID\&quot;: \&quot;District\&quot;,\r\n    \&quot;PurchaseDate\&quot;: \&quot;2018-08-18T00:00:00.000Z\&quot;,\r\n    \&quot;EstimatedDeliveryDate\&quot;: null,\r\n    \&quot;Notes\&quot;: \&quot;Hayes Integration 2018-8-28 \&quot;,\r\n    \&quot;Other1\&quot;: null,\r\n    \&quot;StateFunding\&quot;: null,\r\n    \&quot;FederalFunding\&quot;: null } ]\r\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>966e7561-f8a9-4561-b37d-42ac1590c513</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${TIPWebAPI_Bearer}</value>
      <webElementGuid>8e03d70a-8acc-4b31-b380-d54f709b4773</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${TIPWebAPI_Url}/api/Integrations/PurchaseOrders/AddPurchaseOrderHeaders</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TIPWebAPI_Url</defaultValue>
      <description></description>
      <id>375e661a-b75a-4628-bd6b-954d2c849583</id>
      <masked>false</masked>
      <name>TIPWebAPI_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TIPWebAPI_Bearer</defaultValue>
      <description></description>
      <id>a05e5335-7d6e-41f3-96a3-523071e240f0</id>
      <masked>false</masked>
      <name>TIPWebAPI_Bearer</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>