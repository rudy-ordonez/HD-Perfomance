import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Logging/Login - User'), [('url') : GlobalVariable.url, ('user') : GlobalVariable.user, ('password') : GlobalVariable.adminpass], 
    FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/h1_Create Ticket'))

WebUI.click(findTestObject('IT Fields - User/buttons/div_A - Testing PT'))

WebUI.click(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/div_Proceed Without Sub-Problem Type'))

WebUI.setText(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/input_Summary_summary'), 
    'Automated testing - User')

WebUI.setText(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/textarea_Description_description'), 
    'Katalon automated testing')

WebUI.click(findTestObject('IT Fields - User/buttons/Next'))

WebUI.click(findTestObject('IT Fields - User/buttons/svg_Device Details Next'))

WebUI.click(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/a_Clickheretorecordittothisticket'))

//WebUI.click(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/a_NotListed'))
'Tag Number can vary'
WebUI.setText(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/input_TagNumber_assetTagNumber'), 
    'LSE043175')

WebUI.click(findTestObject('IT Fields - User/buttons/svg_Verify Tag'))

WebUI.click(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/button_Yes'))

WebUI.click(findTestObject('IT Fields - User/buttons/svg_Device Details Next'))

WebUI.click(findTestObject('IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/span_Site_Clear'))

