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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import groovy.time.TimeCategory as TimeCategory
import groovy.time.TimeDuration as TimeDuration

f = new File('C://Temp/grid_perf.csv')

WebUI.openBrowser('')

WebUI.navigateToUrl(url)

WebUI.click(findTestObject('Help Desk/Login/button_Sign In'))

WebUI.setText(findTestObject('Help Desk/Login/input_Username_login'), username)

WebUI.setText(findTestObject('Help Desk/Login/input_Password_pwd'), password)

WebUI.click(findTestObject('Help Desk/Login/button_Log On'))

WebUI.waitForElementClickable(findTestObject('Help Desk/Login/dropdown/span_--- Sort By ---dropdown'), 10)

WebUI.click(findTestObject('Help Desk/Tickets Grid Perf/b_Tickets'))

WebUI.click(findTestObject('Help Desk/Tickets Grid Perf/a_All Tickets'))

WebUI.delay(6)

WebUI.click(findTestObject('Help Desk/Tickets Grid Perf/Dropdown/select_View All Tickets View Active Tickets View Inactive Tickets'), 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.delay(2)

WebUI.click(findTestObject('Help Desk/Tickets Grid Perf/Dropdown/select_All Tickets'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.delay(2)

WebUI.click(findTestObject('Help Desk/Tickets Grid Perf/select_10  25  50'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.selectOptionByValue(findTestObject('Help Desk/Tickets Grid Perf/select_10  25  50'), '50', true)

WebUI.delay(5)

for (int i = 0; i < 5; i++) {
    // long a1 = System.currentTimeMillis()
    start = new Date()

    WebUI.waitForElementPresent(findTestObject('Help Desk/Tickets Grid Perf/Priorities/div_Low'), 10)

    // WebUI.waitForElementVisible(findTestObject('Help Desk/Tickets Grid Perf/status/td_ Open'), 0)
    // long a2 = System.currentTimeMillis()
    end = new Date()

    // (((f << new Date(a1).format('yyyy-MM-dd HH:mm:ss.SSS')) << ',') << ((a2 - a1) / 1000)) << '\r\n'
    TimeDuration td = TimeCategory.minus(end, start)

    (f << '\n') << td // Appends result to file

    WebUI.refresh()
}

WebUI.click(findTestObject('Help Desk/Logout/a_Rudy Ordonez'))

WebUI.click(findTestObject('Help Desk/Logout/a_Log Out'))

WebUI.closeBrowser()

