<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT UpdateCustomField</name>
   <tag></tag>
   <elementGuidId>5b0ba895-86c1-4d07-aaf0-5e318f9e7ac4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;customFieldID\&quot;: 11,\n    \&quot;customFieldTypeID\&quot;: 6,\n    \&quot;fieldName\&quot;: \&quot;DropDownFlip\&quot;,\n    \&quot;description\&quot;: \&quot;Drop down Flip Field, starts multi-select\&quot;,\n    \&quot;required\&quot;: false,\n    \&quot;userID\&quot;: 0,\n    \&quot;optionsList\&quot;: [\n        {\n            \&quot;customFieldListValue\&quot;: \&quot;a\&quot;\n        },\n        {\n            \&quot;customFieldListValue\&quot;: \&quot;b\&quot;\n        },\n        {\n            \&quot;customFieldListValue\&quot;: \&quot;c\&quot;\n        },\n        {\n            \&quot;customFieldListValue\&quot;: \&quot;d\&quot;\n        },\n        {\n            \&quot;customFieldListValue\&quot;: \&quot;e\&quot;\n        },\n        {\n            \&quot;customFieldListValue\&quot;: \&quot;f\&quot;\n        },\n        {\n            \&quot;customFieldListValue\&quot;: \&quot;i\&quot;\n        },\n        {\n            \&quot;customFieldListValue\&quot;: \&quot;j\&quot;\n        }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${HHD_API_Bearer}</value>
      <webElementGuid>f8b1393d-af64-43b0-b7f2-7b8b957786a8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a85e0c04-05d4-46ab-842a-9913d0baddd9</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${HHD_API_Url}/api/CustomFields/UpdateCustomField</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Url</defaultValue>
      <description></description>
      <id>faa88465-f39d-4748-8132-49a9440ac9cc</id>
      <masked>false</masked>
      <name>HHD_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Bearer</defaultValue>
      <description></description>
      <id>90a2df5b-80f8-4a1d-aadc-cf55ad9127d4</id>
      <masked>false</masked>
      <name>HHD_API_Bearer</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
