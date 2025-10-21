#[doc = "Register `OADDR1` reader"]
pub type R = crate::R<OADDR1_SPEC>;
#[doc = "Register `OADDR1` writer"]
pub type W = crate::W<OADDR1_SPEC>;
#[doc = "Field `ADDR1` reader - Interface address"]
pub type ADDR1_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR1` writer - Interface address"]
pub type ADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
#[doc = "Field `ADDR1MODE` reader - Own Address mode"]
pub type ADDR1MODE_R = crate::BitReader;
#[doc = "Field `ADDR1MODE` writer - Own Address mode"]
pub type ADDR1MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR1EN` reader - Own address 1 enable"]
pub type ADDR1EN_R = crate::BitReader;
#[doc = "Field `ADDR1EN` writer - Own address 1 enable"]
pub type ADDR1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address mode"]
    #[inline(always)]
    pub fn addr1mode(&self) -> ADDR1MODE_R {
        ADDR1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Own address 1 enable"]
    #[inline(always)]
    pub fn addr1en(&self) -> ADDR1EN_R {
        ADDR1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OADDR1")
            .field("addr1", &self.addr1())
            .field("addr1mode", &self.addr1mode())
            .field("addr1en", &self.addr1en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn addr1(&mut self) -> ADDR1_W<'_, OADDR1_SPEC> {
        ADDR1_W::new(self, 0)
    }
    #[doc = "Bit 10 - Own Address mode"]
    #[inline(always)]
    pub fn addr1mode(&mut self) -> ADDR1MODE_W<'_, OADDR1_SPEC> {
        ADDR1MODE_W::new(self, 10)
    }
    #[doc = "Bit 15 - Own address 1 enable"]
    #[inline(always)]
    pub fn addr1en(&mut self) -> ADDR1EN_W<'_, OADDR1_SPEC> {
        ADDR1EN_W::new(self, 15)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`oaddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oaddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OADDR1_SPEC;
impl crate::RegisterSpec for OADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oaddr1::R`](R) reader structure"]
impl crate::Readable for OADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oaddr1::W`](W) writer structure"]
impl crate::Writable for OADDR1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OADDR1 to value 0"]
impl crate::Resettable for OADDR1_SPEC {}
