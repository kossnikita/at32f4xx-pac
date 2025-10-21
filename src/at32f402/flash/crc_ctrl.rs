#[doc = "Register `CRC_CTRL` reader"]
pub type R = crate::R<CRC_CTRL_SPEC>;
#[doc = "Register `CRC_CTRL` writer"]
pub type W = crate::W<CRC_CTRL_SPEC>;
#[doc = "Field `CRC_SN` reader - CRC sector numbler"]
pub type CRC_SN_R = crate::FieldReader<u16>;
#[doc = "Field `CRC_SN` writer - CRC sector numbler"]
pub type CRC_SN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CRC_STRT` writer - CRC start"]
pub type CRC_STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - CRC sector numbler"]
    #[inline(always)]
    pub fn crc_sn(&self) -> CRC_SN_R {
        CRC_SN_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_CTRL")
            .field("crc_sn", &self.crc_sn())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC sector numbler"]
    #[inline(always)]
    pub fn crc_sn(&mut self) -> CRC_SN_W<'_, CRC_CTRL_SPEC> {
        CRC_SN_W::new(self, 0)
    }
    #[doc = "Bit 16 - CRC start"]
    #[inline(always)]
    pub fn crc_strt(&mut self) -> CRC_STRT_W<'_, CRC_CTRL_SPEC> {
        CRC_STRT_W::new(self, 16)
    }
}
#[doc = "Flash CRC controll register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_CTRL_SPEC;
impl crate::RegisterSpec for CRC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_ctrl::R`](R) reader structure"]
impl crate::Readable for CRC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crc_ctrl::W`](W) writer structure"]
impl crate::Writable for CRC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRC_CTRL to value 0"]
impl crate::Resettable for CRC_CTRL_SPEC {}
