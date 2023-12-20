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

WebUI.waitForElementPresent(findTestObject('Parts/Add Record/button_Add Record'), 10)

WebUI.click(findTestObject('Object Repository/Parts/Update/HH-1121/a_Page 2'))

WebUI.click(findTestObject('Object Repository/Parts/Update/HH-1121/a_Select Part Area_k-icon k-plus'))

WebUI.click(findTestObject('Parts/Update/HH-1121/button_Add Inventory'))

WebUI.click(findTestObject('Object Repository/Parts/Update/HH-1121/input_Assign Site Name'))

WebUI.click(findTestObject('Object Repository/Parts/Update/HH-1121/svg_Save Assign Sites'))

WebUI.verifyElementPresent(findTestObject('Parts/Update/HH-1121/a_Page 2'), 4)

WebUI.delay(4)

WebUI.click(findTestObject('Parts/Add Record/button_RefreshRefresh'))

